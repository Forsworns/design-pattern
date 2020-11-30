mod paper;
mod question_bank;
mod question_bank_controller;

use paper::{ChoiceQuestion, EssayQuestion, Question};
use question_bank::QuestionBank;
use std::collections::HashMap;

pub use question_bank_controller::QuestionBankController;

use rand::seq::SliceRandom;
use rand::thread_rng;
