use std::rc::Rc;

use crate::{
    client::websocket::WsClient,
    error::Error,
    model::embedded::accelerator::{Project, ProjectList},
};

pub struct AcceleratorApi {
    pub client: Rc<WsClient>,
}

impl AcceleratorApi {
    pub fn new(client: Rc<WsClient>) -> Self {
        Self { client }
    }

    pub async fn get_all(&self, page_index: u8, page_size: u8) -> Result<ProjectList, Error> {
        let response = self
            .client
            .as_ref()
            .send_request(
                "embedded.accelerator.getAll",
                vec![
                    serde_json::to_value(page_index)?,
                    serde_json::to_value(page_size)?,
                ],
            )
            .await?;

        let pl: ProjectList = ProjectList::from_json(response)?;
        Ok(pl)
    }

    pub async fn get_project_by_id(&self, id: &str) -> Result<Project, Error> {
        let response = self
            .client
            .as_ref()
            .send_request(
                "embedded.accelerator.getProjectById",
                vec![serde_json::to_value(id)?],
            )
            .await?;
        let p: Project = Project::from_json(response)?;
        Ok(p)
    }
}
