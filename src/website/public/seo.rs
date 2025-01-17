pub struct Meta {
    pub meta_title: String,
    pub meta_description: String,
    pub meta_keywords: String,
    pub meta_author: String,
    pub meta_url: String,
    pub twitter: TwitterMetadata,
}

impl Meta {
    pub fn new(
        meta_title: String,
        meta_description: String,
        meta_keywords: String,
        meta_author: String,
        meta_url: String,
        meta_image: String,
    ) -> Meta {
        let twitter = TwitterMetadata::new(
            "elerem.com".into(),
            meta_title.clone(),
            meta_description.clone(),
            meta_author.clone(),
            meta_image,
        );
        Self {
            meta_title: meta_title.clone(),
            meta_description: meta_description.clone(),
            meta_keywords: meta_keywords.clone(),
            meta_author: meta_author.clone(),
            meta_url: meta_url.into(),
            twitter,
        }
    }
}

pub struct TwitterMetadata {
    pub site: String,
    pub title: String,
    pub description: String,
    pub creator: String,
    pub image: String,
}

impl TwitterMetadata {
    pub fn new(
        site: String,
        title: String,
        description: String,
        creator: String,
        image: String,
    ) -> Self {
        TwitterMetadata {
            site: site.into(),
            title: title.into(),
            description: description.into(),
            creator: creator.into(),
            image: image.into(),
        }
    }
}
