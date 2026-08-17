use super::context::PromptContext;

pub fn assemble(ctx: &PromptContext) -> String {
    assemble_with_template(ctx, QwenTemplate)
}

pub fn assemble_with_template(ctx: &PromptContext, template: impl PromptTemplate) -> String {
    let mut result = String::new();
    
    result.push_str(&template.system_start());
    result.push_str(&ctx.system);
    result.push_str(&template.system_end());
    
    for msg in &ctx.history {
        result.push_str(&template.user_start());
        result.push_str(msg);
        result.push_str(&template.user_end());
    }
    
    result.push_str(&template.user_start());
    result.push_str(&ctx.user);
    result.push_str(&template.user_end());
    
    result.push_str(&template.assistant_start());
    
    result
}

pub trait PromptTemplate {
    fn system_start(&self) -> &str;
    fn system_end(&self) -> &str;
    fn user_start(&self) -> &str;
    fn user_end(&self) -> &str;
    fn assistant_start(&self) -> &str;
}

pub struct QwenTemplate;

impl PromptTemplate for QwenTemplate {
    fn system_start(&self) -> &str {
        "<|im_start|>system\n"
    }

    fn system_end(&self) -> &str {
        "<|im_end|>\n"
    }

    fn user_start(&self) -> &str {
        "<|im_start|>user\n"
    }

    fn user_end(&self) -> &str {
        "<|im_end|>\n"
    }

    fn assistant_start(&self) -> &str {
        "<|im_start|>assistant\n"
    }
}