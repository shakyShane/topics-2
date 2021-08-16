use typescript_definitions::TypeScriptifyTrait;

fn main() {
    println!("{}", ir::Ir::type_script_ify());
    println!("{}", ir::IrItem::type_script_ify());
    println!("{}", ir::Action::type_script_ify());
    println!("{}", ir::Instruction::type_script_ify());
    println!("{}", ir::Markdown::type_script_ify());
    println!("{}", ir::DependencyList::type_script_ify());
    println!("{}", ir::NamedRefList::type_script_ify());
    println!("{}", ir::NamedRef::type_script_ify());
    println!("{}", ir::IdRef::type_script_ify());
    println!("{}", ir::Step::type_script_ify());
    println!("{}", ir::Command::type_script_ify());
    println!("{}", ir::CommandDefinition::type_script_ify());
    println!("{}", ir::CommandConfig::type_script_ify());
    println!("{}", ir::CommandParams::type_script_ify());
    println!("{}", ir::InputLanguage::type_script_ify());
    println!("{}", ir::Config::type_script_ify());
    println!("{}", ir::ConfigDefinition::type_script_ify());
    println!("{}", ir::ConfigParams::type_script_ify());
}
