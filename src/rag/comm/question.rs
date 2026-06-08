#[derive(Debug, Clone)]
pub struct Question {
    system_prompt: String,
    question: String,
    context: Vec<String>,
    model: String,
}

impl From<String> for Question {
    fn from(value: String) -> Self {
        Self {
            system_prompt: "You are a helpful assistant. Answer users question based on provided context.".to_owned(),
            question: value,
            context: vec![],
            model: "hf.co/unsloth/Qwen3-30B-A3B-Instruct-2507-GGUF:UD-Q4_K_XL".to_owned(),
        }
    }
}

impl From<&str> for Question {
    fn from(value: &str) -> Self {
        Self {
            system_prompt: "You are a helpful assistant. Answer users question based on provided context.".to_owned(),
            question: value.to_owned(),
            context: vec![],
            model: "hf.co/unsloth/Qwen3-30B-A3B-Instruct-2507-GGUF:UD-Q4_K_XL".to_owned(),
        }
    }
}

impl Question {
    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }

    pub fn user_content(&self) -> String {
        let context = if self.context.is_empty() {
            "".to_string()
        } else {
            self.context.join("\n")
        };

        format!("{}\n{}", self.question, context)
    }

    pub fn set_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }

    pub fn set_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub fn set_question(mut self, question: &str) -> Self {
        self.question = question.to_string();
        self
    }

    pub fn set_context(mut self, context: Vec<String>) -> Self {
        self.context = context;
        self
    }
}
