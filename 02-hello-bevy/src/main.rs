use bevy::prelude::*;
///ECS (实体-组件-系统) 是一种编程范式，通常用于游戏开发和实时渲染等领域。
/// Entity: 实体是游戏世界中的一个对象，可以是一个角色、道具、场景元素等。实体通常是唯一的，并且可以拥有多个组件。
/// Component: 组件是附加到实体上的数据或属性。组件可以是任何类型的数据，例如位置、速度、颜色等。组件通常是轻量级的，并且可以被多个实体共享。
/// System: 系统是处理实体和组件的逻辑。系统通常是一个函数，它会查询所有具有特定组件的实体，并对它们进行操作。系统可以是并行的，也可以是串行的。
///用组件存储数据，用实体关联组件，用系统处理逻辑
/// Component 
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);


///System
fn print_name(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("{}", name.0);
    }
}
fn setup(mut commands: Commands) {
    // Entity
    commands.spawn((
        Person,
        Name("gg".to_string()),
    ));
    commands.spawn((
        Person,
        Name("ll".to_string()),
    ));
    
}



// 程序入口点
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_name)
        .run();
}