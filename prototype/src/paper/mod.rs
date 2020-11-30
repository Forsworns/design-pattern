mod choice_question;
mod essay_question;

pub use choice_question::ChoiceQuestion;
pub use essay_question::EssayQuestion;

// idea of super trait, though this `Question` trait is only used for annotation...
pub trait Question: ToString + QuestionClone {}

pub trait QuestionClone {
    fn clone_box(&self) -> Box<dyn Question>;
}

impl<T> QuestionClone for T
where
    T: 'static + Question + Clone,
{
    fn clone_box(&self) -> Box<dyn Question> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Question> {
    fn clone(&self) -> Box<dyn Question> {
        self.clone_box()
    }
}
