#![allow(missing_docs)]
mod accessor_alternative;
pub use accessor_alternative::AccessorAlternative;
mod and_preference;
pub use and_preference::AndPreference;
mod anonymous_constructor;
pub use anonymous_constructor::AnonymousConstructor;
mod assignment_to_call;
pub use assignment_to_call::AssignmentToCall;
mod bool_equality;
pub use bool_equality::BoolEquality;
mod constructor_without_new;
pub use constructor_without_new::ConstructorWithoutNew;
mod deprecated;
pub use deprecated::Deprecated;
mod draw_sprite;
pub use draw_sprite::DrawSprite;
mod draw_text;
pub use draw_text::DrawText;
mod english_flavor_violation;
pub use english_flavor_violation::EnglishFlavorViolation;
mod exit;
pub use exit::Exit;
mod global;
pub use global::Global;
mod missing_case_member;
pub use missing_case_member::MissingCaseMember;
mod missing_default_case;
pub use missing_default_case::MissingDefaultCase;
mod mod_preference;
pub use mod_preference::ModPreference;
mod multi_var_declaration;
pub use multi_var_declaration::MultiVarDeclaration;
mod no_space_begining_comment;
pub use no_space_begining_comment::NoSpaceBeginingComment;
mod non_constant_default_parameter;
pub use non_constant_default_parameter::NonConstantDefaultParameter;
mod non_pascal_case;
pub use non_pascal_case::NonPascalCase;
mod non_scream_case;
pub use non_scream_case::NonScreamCase;
mod not_preference;
pub use not_preference::NotPreference;
mod or_preference;
pub use or_preference::OrPreference;
mod room_goto;
pub use room_goto::RoomGoto;
mod show_debug_message;
pub use show_debug_message::ShowDebugMessage;
mod single_switch_case;
pub use single_switch_case::SingleSwitchCase;
mod statement_parenthetical_preference;
pub use statement_parenthetical_preference::StatementParentheticalPreference;
mod suspicious_constant_usage;
pub use suspicious_constant_usage::SuspicousConstantUsage;
mod todo;
pub use todo::Todo;
mod too_many_arguments;
pub use too_many_arguments::TooManyArguments;
mod too_many_lines;
pub use too_many_lines::TooManyLines;
mod try_catch;
pub use try_catch::TryCatch;
mod var_prefix_violation;
pub use var_prefix_violation::VarPrefixViolation;
mod with_loop;
pub use with_loop::WithLoop;
