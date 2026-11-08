use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_clicks, click_system))
        .run();
}

#[derive(Resource)]
struct ClickerData {
    tea: u64,
}

#[derive(Component)]
struct ClickText;

fn setup(mut commands: Commands) {
    commands.insert_resource(ClickerData { tea: 0 });
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        children![
            (Text::new("The amount of tea has been prepared")),
            (
                Text::new("0"),
                ClickText,
                Node {
                    padding: UiRect {
                        top: Val::Px(30.0),
                        ..default()
                    },
                    ..default()
                }
            )
        ],
    ));
}

fn update_clicks(data: Res<ClickerData>, mut q: Query<&mut Text, With<ClickText>>) {
    let Ok(mut text) = q.single_mut() else {
        return;
    };
    text.0 = data.tea.to_string();
}

fn click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut data: ResMut<ClickerData>
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        data.tea += 1;
    }
}