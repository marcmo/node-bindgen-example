pub trait Source {
    fn get_content(&self) -> Vec<String>;
}

pub struct TextSource {
    pub file: String,
}

impl Source for TextSource {
    fn get_content(&self) -> Vec<String> {
        vec!["Hello".to_string(), "World".to_string()]
    }
}

pub struct Grabber<T: Source> {
    pub source: T,
}

pub trait GrabTrait {
    fn grab_content(&self) -> Vec<String>;
}

impl<T> GrabTrait for Grabber<T>
where
    T: Source,
{
    fn grab_content(&self) -> Vec<String> {
        self.source.get_content()
    }
}
