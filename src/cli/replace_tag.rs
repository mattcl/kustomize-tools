use std::{collections::HashSet, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::Args;
use serde::{Deserialize, Serialize};

fn multidoc_deserialize(data: &str) -> Result<Vec<serde_yaml::Value>, serde_yaml::Error> {
    let mut docs = Vec::new();
    for de in serde_yaml::Deserializer::from_str(data) {
        docs.push(serde_yaml::Value::deserialize(de)?);
    }
    Ok(docs)
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Kustomization {
    pub images: Vec<KustomizationImage>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KustomizationImage {
    pub name: String,
    pub new_name: Option<String>,
    pub new_tag: String,
}

/// Replaces the newTag for the specified image in the kustomization file.
///
/// This will do nothing if the desired tag is already the tag specified in the
/// kustomization file.
///
/// This will error if it's the case that another image has the same tag as the
/// existing tag for the image we're trying to replace.
#[derive(Debug, Clone, Args)]
pub struct ReplaceTag {
    file: PathBuf,

    /// The target image.
    #[arg(short, long)]
    image: String,

    /// The tag to set.
    #[arg(short, long)]
    tag: String,
}

impl ReplaceTag {
    pub fn run(&self) -> Result<()> {
        // we're going to read the contents of the file then write them back to
        // disk with just the tag replaced. This _should_ preserve comments and
        // other formatting.
        let mut contents =
            std::fs::read_to_string(&self.file).context("Could not read kustomization file")?;
        let mut docs =
            multidoc_deserialize(&contents).context("Failed to read kustomization file as YAML")?;

        let doc = docs
            .pop()
            .ok_or_else(|| anyhow!("Could not get document from YAML file"))?;

        let kustomization: Kustomization =
            serde_yaml::from_value(doc).context("Could not deserialize as a Kustomization file")?;

        // find the image (and tag) we want to replace
        let img = kustomization
            .images
            .iter()
            .find(|img| img.name == self.image)
            .ok_or_else(|| anyhow!("Could not find the specified image in the file"))?;

        // exit early if there would be no changes
        if img.new_tag == self.tag {
            println!(
                "  The tag '{}' is already specified for '{}'",
                &img.new_tag, &img.name
            );
            return Ok(());
        }

        // sanity check to see if we have a duplicate
        let mut seen: HashSet<&str> = HashSet::default();

        for image in kustomization.images.iter() {
            if seen.contains(&image.new_tag.as_str()) && img.name == image.name {
                bail!("Kustomization file contains another image with the same tag as the one we're tyring to replace.")
            }
            seen.insert(&image.new_tag);
        }

        println!(
            "  Replacing tag '{}' with '{}' for image '{}'",
            &img.new_tag, &self.tag, &img.name
        );

        // This really only works if the tag we want to replace is unique to the
        // kustomization file, hence the previous check.
        contents = contents.replace(&img.new_tag, &self.tag);

        std::fs::write(&self.file, &contents).context("Failed to modify kustomization file")?;

        Ok(())
    }
}
