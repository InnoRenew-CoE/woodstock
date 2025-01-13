use ollama_rs::generation::completion::request::GenerationRequest;


pub struct Questiton {
    system_prompt: String,
    question: String,
    context: Vec<String>,
    model: String,
}

impl From<String> for Questiton {
    fn from(value: String) -> Self {
        Self {
            system_prompt: "You are a helpful assistant. Answer users question based on provided context.".to_owned(),
            question: value,
            context: vec![],
            model: "mistral-nemo".to_owned(),
        }
    }
}


impl From<&str> for Questiton {
    fn from(value: &str) -> Self {
        Self {
            system_prompt: "You are a helpful assistant. Answer users question based on provided context.".to_owned(),
            question: value.to_owned(),
            context: vec![],
            model: "mistral-nemo".to_owned(),
        }
    }
}


impl Into<GenerationRequest> for Questiton {
    fn into(self) -> GenerationRequest {
        let mut context = "No context. Answer baed on your knowledge.".to_string();
        if !self.context.is_empty() {
            context = self.context.join("\n");
        }
        let final_prompt = format!(
            "{}\n{}\nContext:\n{}", 
            self.system_prompt,
            self.question,
            context
        );
        GenerationRequest::new(self.model, final_prompt)
    }
}