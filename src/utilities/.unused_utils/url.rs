#[derive(Resource)]
pub(crate) struct OpenLinkResource(pub Box<dyn Fn(&str) + Sync + Send + 'static>);
