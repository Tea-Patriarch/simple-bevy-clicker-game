use bevy::{
    prelude::*,
    ui::BorderRadius,
    ui_widgets::{Activate, Button, observe},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(ClickerData { tea: 0 })
        .add_message::<ClickEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_clicks)
        .run();
}

#[derive(Resource)]
struct ClickerData {
    tea: u64,
}

#[derive(Component)]
struct ClickText;

#[derive(Message)]
struct ClickEvent;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
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
                        bottom: Val::Px(60.0),
                        ..default()
                    },
                    ..default()
                }
            ),
            (
                //Increase tea count button
                Button,
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(0.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.85, 0.85, 0.85)),
                children![(
                    Text::new("Brew tea"),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        weight: FontWeight::BOLD,
                        ..default()
                    },
                    TextColor(Color::srgb_u8(43, 44, 47))
                ),],
                observe(
                    |_activate: On<Activate>, mut writer: MessageWriter<ClickEvent>| {
                        writer.write(ClickEvent);
                    }
                ),
            )
        ],
    ));
}

fn update_clicks(
    mut data: ResMut<ClickerData>,
    mut q: Query<&mut Text, With<ClickText>>,
    mut message_reader: MessageReader<ClickEvent>,
) {
    for _ in message_reader.read() {
        data.tea += 1;
        for mut i in &mut q {
            i.0 = data.tea.to_string()
        }
    }
}
