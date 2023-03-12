use bevy::math::vec2;
use rusty_engine::{prelude::*};
use std::time::Instant;

struct GameState {
    luotchoi:bool,
    win:bool,
}
fn main() 
{
    let mut game = Game::new();
    let game_state= GameState { luotchoi:true,win:false,};
    
    
    let a = game.add_sprite("khung", SpritePreset::Khung);
    a.layer=1.0;

    let a = game.add_sprite("black1", SpritePreset::Black);
    a.translation=vec2(-175.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black2", SpritePreset::Black);
    a.translation=vec2(-175.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black3", SpritePreset::Black);
    a.translation=vec2(-175.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black4", SpritePreset::Black);
    a.translation=vec2(-175.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black5", SpritePreset::Black);
    a.translation=vec2(-175.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black6", SpritePreset::Black);
    a.translation=vec2(-175.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black7", SpritePreset::Black);
    a.translation=vec2(-175.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black8", SpritePreset::Black);
    a.translation=vec2(-175.0,  175.0);
    a.num=2;a.collision=true;
    
    let a = game.add_sprite("black9", SpritePreset::Black);
    a.translation=vec2(-125.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black10", SpritePreset::Black);
    a.translation=vec2(-125.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black11", SpritePreset::Black);
    a.translation=vec2(-125.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black12", SpritePreset::Black);
    a.translation=vec2(-125.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black13", SpritePreset::Black);
    a.translation=vec2(-125.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black14", SpritePreset::Black);
    a.translation=vec2(-125.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black15", SpritePreset::Black);
    a.translation=vec2(-125.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black16", SpritePreset::Black);
    a.translation=vec2(-125.0,  175.0);
    a.num=2;a.collision=true;

    let a = game.add_sprite("black17", SpritePreset::Black);
    a.translation=vec2(-75.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black18", SpritePreset::Black);
    a.translation=vec2(-75.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black19", SpritePreset::Black);
    a.translation=vec2(-75.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black20", SpritePreset::Black);
    a.translation=vec2(-75.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black21", SpritePreset::Black);
    a.translation=vec2(-75.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black22", SpritePreset::Black);
    a.translation=vec2(-75.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black23", SpritePreset::Black);
    a.translation=vec2(-75.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black24", SpritePreset::Black);
    a.translation=vec2(-75.0,  175.0);
    a.num=2;a.collision=true;

    let a = game.add_sprite("black25", SpritePreset::Black);
    a.translation=vec2(-25.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black26", SpritePreset::Black);
    a.translation=vec2(-25.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black27", SpritePreset::Black);
    a.translation=vec2(-25.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black28", SpritePreset::Black);
    a.translation=vec2(-25.0, -25.0);
    a.num=2;a.collision=true;
    a.layer=10.0;
    let a = game.add_sprite("black29", SpritePreset::Black);
    a.translation=vec2(-25.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black30", SpritePreset::Black);
    a.translation=vec2(-25.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black31", SpritePreset::Black);
    a.translation=vec2(-25.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black32", SpritePreset::Black);
    a.translation=vec2(-25.0,  175.0);
    a.num=2;a.collision=true;

    let a = game.add_sprite("black33", SpritePreset::Black);
    a.translation=vec2(25.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black34", SpritePreset::Black);
    a.translation=vec2(25.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black35", SpritePreset::Black);
    a.translation=vec2(25.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black36", SpritePreset::Black);
    a.translation=vec2(25.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black37", SpritePreset::Black);
    a.translation=vec2(25.0,  25.0);
    a.num=2;a.collision=true;
    a.layer=10.0;
    let a = game.add_sprite("black38", SpritePreset::Black);
    a.translation=vec2(25.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black39", SpritePreset::Black);
    a.translation=vec2(25.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black40", SpritePreset::Black);
    a.translation=vec2(25.0,  175.0);
    a.num=2;a.collision=true;

    let a = game.add_sprite("black41", SpritePreset::Black);
    a.translation=vec2(75.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black42", SpritePreset::Black);
    a.translation=vec2(75.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black43", SpritePreset::Black);
    a.translation=vec2(75.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black44", SpritePreset::Black);
    a.translation=vec2(75.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black45", SpritePreset::Black);
    a.translation=vec2(75.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black46", SpritePreset::Black);
    a.translation=vec2(75.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black47", SpritePreset::Black);
    a.translation=vec2(75.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black48", SpritePreset::Black);
    a.translation=vec2(75.0,  175.0);
    a.num=2;a.collision=true;

    let a = game.add_sprite("black49", SpritePreset::Black);
    a.translation=vec2(125.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black50", SpritePreset::Black);
    a.translation=vec2(125.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black51", SpritePreset::Black);
    a.translation=vec2(125.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black52", SpritePreset::Black);
    a.translation=vec2(125.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black53", SpritePreset::Black);
    a.translation=vec2(125.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black54", SpritePreset::Black);
    a.translation=vec2(125.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black55", SpritePreset::Black);
    a.translation=vec2(125.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black56", SpritePreset::Black);
    a.translation=vec2(125.0,  175.0);
    a.num=2;a.collision=true;

    let a = game.add_sprite("black57", SpritePreset::Black);
    a.translation=vec2(175.0, -175.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black58", SpritePreset::Black);
    a.translation=vec2(175.0, -125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black59", SpritePreset::Black);
    a.translation=vec2(175.0, -75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black60", SpritePreset::Black);
    a.translation=vec2(175.0, -25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black61", SpritePreset::Black);
    a.translation=vec2(175.0,  25.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black62", SpritePreset::Black);
    a.translation=vec2(175.0,  75.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black63", SpritePreset::Black);
    a.translation=vec2(175.0,  125.0);
    a.num=2;a.collision=true;
    let a = game.add_sprite("black64", SpritePreset::Black);
    a.translation=vec2(175.0,  175.0);
    a.num=2;a.collision=true;


    let a = game.add_sprite("white1", SpritePreset::White);
    a.translation=vec2(-175.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white2", SpritePreset::White);
    a.translation=vec2(-175.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white3", SpritePreset::White);
    a.translation=vec2(-175.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white4", SpritePreset::White);
    a.translation=vec2(-175.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white5", SpritePreset::White);
    a.translation=vec2(-175.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white6", SpritePreset::White);
    a.translation=vec2(-175.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white7", SpritePreset::White);
    a.translation=vec2(-175.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white8", SpritePreset::White);
    a.translation=vec2(-175.0,  175.0);
    a.num=1;a.collision=true;
    
    let a = game.add_sprite("white9", SpritePreset::White);
    a.translation=vec2(-125.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white10", SpritePreset::White);
    a.translation=vec2(-125.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white11", SpritePreset::White);
    a.translation=vec2(-125.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white12", SpritePreset::White);
    a.translation=vec2(-125.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white13", SpritePreset::White);
    a.translation=vec2(-125.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white14", SpritePreset::White);
    a.translation=vec2(-125.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white15", SpritePreset::White);
    a.translation=vec2(-125.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white16", SpritePreset::White);
    a.translation=vec2(-125.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("white17", SpritePreset::White);
    a.translation=vec2(-75.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white18", SpritePreset::White);
    a.translation=vec2(-75.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white19", SpritePreset::White);
    a.translation=vec2(-75.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white20", SpritePreset::White);
    a.translation=vec2(-75.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white21", SpritePreset::White);
    a.translation=vec2(-75.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white22", SpritePreset::White);
    a.translation=vec2(-75.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white23", SpritePreset::White);
    a.translation=vec2(-75.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white24", SpritePreset::White);
    a.translation=vec2(-75.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("white25", SpritePreset::White);
    a.translation=vec2(-25.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white26", SpritePreset::White);
    a.translation=vec2(-25.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white27", SpritePreset::White);
    a.translation=vec2(-25.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white28", SpritePreset::White);
    a.translation=vec2(-25.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white29", SpritePreset::White);
    a.translation=vec2(-25.0,  25.0);
    a.num=1;a.collision=true;
    a.layer=10.0;
    let a = game.add_sprite("white30", SpritePreset::White);
    a.translation=vec2(-25.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white31", SpritePreset::White);
    a.translation=vec2(-25.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white32", SpritePreset::White);
    a.translation=vec2(-25.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("white33", SpritePreset::White);
    a.translation=vec2(25.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white34", SpritePreset::White);
    a.translation=vec2(25.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white35", SpritePreset::White);
    a.translation=vec2(25.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white36", SpritePreset::White);
    a.translation=vec2(25.0, -25.0);
    a.num=1;a.collision=true;
    a.layer=10.0;
    let a = game.add_sprite("white37", SpritePreset::White);
    a.translation=vec2(25.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white38", SpritePreset::White);
    a.translation=vec2(25.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white39", SpritePreset::White);
    a.translation=vec2(25.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white40", SpritePreset::White);
    a.translation=vec2(25.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("white41", SpritePreset::White);
    a.translation=vec2(75.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white42", SpritePreset::White);
    a.translation=vec2(75.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white43", SpritePreset::White);
    a.translation=vec2(75.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white44", SpritePreset::White);
    a.translation=vec2(75.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white45", SpritePreset::White);
    a.translation=vec2(75.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white46", SpritePreset::White);
    a.translation=vec2(75.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white47", SpritePreset::White);
    a.translation=vec2(75.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white48", SpritePreset::White);
    a.translation=vec2(75.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("white49", SpritePreset::White);
    a.translation=vec2(125.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white50", SpritePreset::White);
    a.translation=vec2(125.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white51", SpritePreset::White);
    a.translation=vec2(125.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white52", SpritePreset::White);
    a.translation=vec2(125.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white53", SpritePreset::White);
    a.translation=vec2(125.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white54", SpritePreset::White);
    a.translation=vec2(125.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white55", SpritePreset::White);
    a.translation=vec2(125.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white56", SpritePreset::White);
    a.translation=vec2(125.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("white57", SpritePreset::White);
    a.translation=vec2(175.0, -175.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white58", SpritePreset::White);
    a.translation=vec2(175.0, -125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white59", SpritePreset::White);
    a.translation=vec2(175.0, -75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white60", SpritePreset::White);
    a.translation=vec2(175.0, -25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white61", SpritePreset::White);
    a.translation=vec2(175.0,  25.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white62", SpritePreset::White);
    a.translation=vec2(175.0,  75.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white63", SpritePreset::White);
    a.translation=vec2(175.0,  125.0);
    a.num=1;a.collision=true;
    let a = game.add_sprite("white64", SpritePreset::White);
    a.translation=vec2(175.0,  175.0);
    a.num=1;a.collision=true;

    let a = game.add_sprite("diem1", SpritePreset::Diem);
    a.translation=vec2(-175.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem2", SpritePreset::Diem);
    a.translation=vec2(-175.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem3", SpritePreset::Diem);
    a.translation=vec2(-175.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem4", SpritePreset::Diem);
    a.translation=vec2(-175.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem5", SpritePreset::Diem);
    a.translation=vec2(-175.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem6", SpritePreset::Diem);
    a.translation=vec2(-175.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem7", SpritePreset::Diem);
    a.translation=vec2(-175.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem8", SpritePreset::Diem);
    a.translation=vec2(-175.0,  175.0);
    a.num=3;a.collision=true;
    
    let a = game.add_sprite("diem9", SpritePreset::Diem);
    a.translation=vec2(-125.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem10", SpritePreset::Diem);
    a.translation=vec2(-125.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem11", SpritePreset::Diem);
    a.translation=vec2(-125.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem12", SpritePreset::Diem);
    a.translation=vec2(-125.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem13", SpritePreset::Diem);
    a.translation=vec2(-125.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem14", SpritePreset::Diem);
    a.translation=vec2(-125.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem15", SpritePreset::Diem);
    a.translation=vec2(-125.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem16", SpritePreset::Diem);
    a.translation=vec2(-125.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("diem17", SpritePreset::Diem);
    a.translation=vec2(-75.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem18", SpritePreset::Diem);
    a.translation=vec2(-75.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem19", SpritePreset::Diem);
    a.translation=vec2(-75.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem20", SpritePreset::Diem);
    a.translation=vec2(-75.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem21", SpritePreset::Diem);
    a.translation=vec2(-75.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem22", SpritePreset::Diem);
    a.translation=vec2(-75.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem23", SpritePreset::Diem);
    a.translation=vec2(-75.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem24", SpritePreset::Diem);
    a.translation=vec2(-75.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("diem25", SpritePreset::Diem);
    a.translation=vec2(-25.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem26", SpritePreset::Diem);
    a.translation=vec2(-25.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem27", SpritePreset::Diem);
    a.translation=vec2(-25.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem28", SpritePreset::Diem);
    a.translation=vec2(-25.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem29", SpritePreset::Diem);
    a.translation=vec2(-25.0,  25.0);
    a.num=3;a.collision=true;
    a.layer=10.0;
    let a = game.add_sprite("diem30", SpritePreset::Diem);
    a.translation=vec2(-25.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem31", SpritePreset::Diem);
    a.translation=vec2(-25.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem32", SpritePreset::Diem);
    a.translation=vec2(-25.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("diem33", SpritePreset::Diem);
    a.translation=vec2(25.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem34", SpritePreset::Diem);
    a.translation=vec2(25.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem35", SpritePreset::Diem);
    a.translation=vec2(25.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem36", SpritePreset::Diem);
    a.translation=vec2(25.0, -25.0);
    a.num=3;a.collision=true;
    a.layer=10.0;
    let a = game.add_sprite("diem37", SpritePreset::Diem);
    a.translation=vec2(25.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem38", SpritePreset::Diem);
    a.translation=vec2(25.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem39", SpritePreset::Diem);
    a.translation=vec2(25.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem40", SpritePreset::Diem);
    a.translation=vec2(25.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("diem41", SpritePreset::Diem);
    a.translation=vec2(75.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem42", SpritePreset::Diem);
    a.translation=vec2(75.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem43", SpritePreset::Diem);
    a.translation=vec2(75.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem44", SpritePreset::Diem);
    a.translation=vec2(75.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem45", SpritePreset::Diem);
    a.translation=vec2(75.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem46", SpritePreset::Diem);
    a.translation=vec2(75.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem47", SpritePreset::Diem);
    a.translation=vec2(75.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem48", SpritePreset::Diem);
    a.translation=vec2(75.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("diem49", SpritePreset::Diem);
    a.translation=vec2(125.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem50", SpritePreset::Diem);
    a.translation=vec2(125.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem51", SpritePreset::Diem);
    a.translation=vec2(125.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem52", SpritePreset::Diem);
    a.translation=vec2(125.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem53", SpritePreset::Diem);
    a.translation=vec2(125.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem54", SpritePreset::Diem);
    a.translation=vec2(125.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem55", SpritePreset::Diem);
    a.translation=vec2(125.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem56", SpritePreset::Diem);
    a.translation=vec2(125.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("diem57", SpritePreset::Diem);
    a.translation=vec2(175.0, -175.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem58", SpritePreset::Diem);
    a.translation=vec2(175.0, -125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem59", SpritePreset::Diem);
    a.translation=vec2(175.0, -75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem60", SpritePreset::Diem);
    a.translation=vec2(175.0, -25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem61", SpritePreset::Diem);
    a.translation=vec2(175.0,  25.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem62", SpritePreset::Diem);
    a.translation=vec2(175.0,  75.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem63", SpritePreset::Diem);
    a.translation=vec2(175.0,  125.0);
    a.num=3;a.collision=true;
    let a = game.add_sprite("diem64", SpritePreset::Diem);
    a.translation=vec2(175.0,  175.0);
    a.num=3;a.collision=true;

    let a = game.add_sprite("point", SpritePreset::Point);
    a.collision=true;
    
    let a = game.add_sprite("newgame", SpritePreset::Newgame);
    a.collision=true;
    a.translation=vec2(350.0, 0.0);
    a.scale=0.4;
    let text1 = game.add_text("text1", "");
    text1.font_size=50.0;
    text1.translation=vec2(-350.0, 100.0);
    let text2 = game.add_text("text2", "");
    text2.font_size=50.0;
    text2.translation=vec2(-350.0, -100.0);
    let mut win = game.add_text("win", "");
    win.font_size = 50.0;
    win.translation.y = 300.0;


    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) 
{
    let win= engine.texts.get_mut("win").unwrap();
    
    //gán 1 sprite vào con trỏ
    if let Some(sprite) = engine.sprites.get_mut("point")
    {
        if let Some(location) = engine.mouse_state.location()
        {
            sprite.translation=location;
        }
    }
    game_state.win=true;
    
    //lấy giá trị ra mảng
    let mut arr = [[0i32; 10]; 10];
    for i in 1..9
    {
        for j in 1..9
        {
            for sprite in engine.sprites.values_mut()
            {
                let k=i as f32;
                let l=j as f32;
                if sprite.translation==vec2(-225.0+k*50.0, -225.0+l*50.0) && sprite.layer==10.0 && sprite.num==1
                {
                    arr[i][j]=1;
                }
                if sprite.translation==vec2(-225.0+k*50.0, -225.0+l*50.0) && sprite.layer==10.0 && sprite.num==2
                {
                    arr[i][j]=2;
                }
            }
        }
    }


    game_state.win=true;
    //check xem có còn nước đi nào ko
    for i in 1..9
    {
        for j in 1..9
        {
            //check quân trắng
            if arr[i][j]==0 && arr[i+1][j]==2
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && arr[i+k][j]==2 && check==true
                {
                    if arr[i+k+1][j]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }
            
            if arr[i][j]==0 && arr[i-1][j]==2
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && arr[i-k][j]==2 && check==true
                {
                    if arr[i-k-1][j]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j+1]==2
            {
                let mut k=1;
                let mut check=true;
                while j+k<8 && arr[i][j+k]==2 && check==true
                {
                    if arr[i][j+k+1]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j-1]==2
            {
                let mut k=1;
                let mut check=true;
                while j-k>1 && arr[i][j-k]==2 && check==true
                {
                    if arr[i][j-k-1]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j+1]==2
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j+k<8 && arr[i+k][j+k]==2 && check==true
                {
                    if arr[i+k+1][j+k+1]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j-1]==2
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j-k>1 && arr[i-k][j-k]==2 && check==true
                {
                    if arr[i-k-1][j-k-1]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j+1]==2
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j+k<8 && arr[i-k][j+k]==2 && check==true
                {
                    if arr[i-k-1][j+k+1]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j-1]==2
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j-k>1 && arr[i+k][j-k]==2 && check==true
                {
                    if arr[i+k+1][j-k-1]==1
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }
            
            //check quân đen
            if arr[i][j]==0 && arr[i+1][j]==1
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && arr[i+k][j]==1 && check==true
                {
                    if arr[i+k+1][j]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }
            
            if arr[i][j]==0 && arr[i-1][j]==1
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && arr[i-k][j]==1 && check==true
                {
                    if arr[i-k-1][j]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j+1]==1
            {
                let mut k=1;
                let mut check=true;
                while j+k<8 && arr[i][j+k]==1 && check==true
                {
                    if arr[i][j+k+1]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j-1]==1
            {
                let mut k=1;
                let mut check=true;
                while j-k>1 && arr[i][j-k]==1 && check==true
                {
                    if arr[i][j-k-1]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j+1]==1
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j+k<8 && arr[i+k][j+k]==1 && check==true
                {
                    if arr[i+k+1][j+k+1]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j-1]==1
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j-k>1 && arr[i-k][j-k]==1 && check==true
                {
                    if arr[i-k-1][j-k-1]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j+1]==1
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j+k<8 && arr[i-k][j+k]==1 && check==true
                {
                    if arr[i-k-1][j+k+1]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j-1]==1
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j-k>1 && arr[i+k][j-k]==1 && check==true
                {
                    if arr[i+k+1][j-k-1]==2
                    {
                        check=false;
                        game_state.win=false;
                    }
                    k+=1;
                }
            }  
        }
    }
    //kiểm tra chiến thắng
    if game_state.win==true
    {
        let mut player=0;
        let mut com=0;
        for i in 1..9
        {
            for j in 1..9
            {
                if arr[i][j]==1
                {
                    player+=1;
                }
                if arr[i][j]==2
                {
                    com+=1;
                }
            }
        }
        if player>com
        {
            win.value="You Win!".into();
        }else
        if player<com
        {
            win.value="You Lost!".into();
        }else
        {
            win.value="Draw!".into();
        }
    }
    else
    {
        win.value="".into();
    }
   
    //đến lượt người chơi đánh
    if game_state.luotchoi==true
    {
        game_state.luotchoi=false;
        //check vị trí xem có khả thi ko
        for i in 1..9
        {
            for j in 1..9
            {
                if arr[i][j]==0 && arr[i+1][j]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while i+k<8 && arr[i+k][j]==2 && check==true
                    {
                        if arr[i+k+1][j]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }
                
                if arr[i][j]==0 && arr[i-1][j]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while i-k>1 && arr[i-k][j]==2 && check==true
                    {
                        if arr[i-k-1][j]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;                                
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i][j+1]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while j+k<8 && arr[i][j+k]==2 && check==true
                    {
                        if arr[i][j+k+1]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;                               
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i][j-1]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while j-k>1 && arr[i][j-k]==2 && check==true
                    {
                        if arr[i][j-k-1]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;                               
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i+1][j+1]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while i+k<8 && j+k<8 && arr[i+k][j+k]==2 && check==true
                    {
                        if arr[i+k+1][j+k+1]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;                              
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i-1][j-1]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while i-k>1 && j-k>1 && arr[i-k][j-k]==2 && check==true
                    {
                        if arr[i-k-1][j-k-1]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;               
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i-1][j+1]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while i-k>1 && j+k<8 && arr[i-k][j+k]==2 && check==true
                    {
                        if arr[i-k-1][j+k+1]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;                           
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i+1][j-1]==2
                {
                    let mut k=1;
                    let mut check=true;
                    while i+k<8 && j-k>1 && arr[i+k][j-k]==2 && check==true
                    {
                        if arr[i+k+1][j-k-1]==1
                        {
                            arr[i][j]=3;
                            for sprite in engine.sprites.values_mut()
                            {
                                let k=i as f32;
                                let l=j as f32;
                                if sprite.translation==vec2(-225.0+50.0*k, -225.0+50.0*l) && sprite.num==1
                                {
                                    sprite.check=true;                 
                                }
                            }
                            check=false;
                            game_state.luotchoi=true;
                        }
                        k+=1;
                    }
                }
            }
        }
        
        // xử lí hành động người chơi đánh 1 nước
        if game_state.luotchoi==true
        {         
            for event in engine.collision_events.drain(..) 
            {
                //nếu va chạm không có "point" thì bỏ qua
                if event.pair.0=="point" || event.pair.1=="point"
                {
                    match event.state 
                    {
                        CollisionState::Begin => 
                        {
                            //đánh dấu những sprite đang va chạm
                            if let Some(sprite) = engine.sprites.get_mut(&event.pair.0)
                            {
                                sprite.mark=true;
                            }
                            if let Some(sprite) = engine.sprites.get_mut(&event.pair.1)
                            {
                                sprite.mark=true;
                            }
                        }
                        CollisionState::End => 
                        {
                            if let Some(sprite) = engine.sprites.get_mut(&event.pair.0)
                            {
                                sprite.mark=false;
                            }
                            if let Some(sprite) = engine.sprites.get_mut(&event.pair.1)
                            {
                                sprite.mark=false;
                            }
                        }
                    }
                }
            }

            if engine.mouse_state.just_pressed(MouseButton::Left)
            {
                for sprite in engine.sprites.values_mut()
                {
                    if sprite.mark==true && sprite.check==true
                    {
                        game_state.luotchoi=false;
                        let i1=(sprite.translation.x+225.0)/50.0;
                        let j1=(sprite.translation.y+225.0)/50.0;
                        let i=i1 as usize;
                        let j=j1 as usize;
                        arr[i][j]=1;
                       
                        //refresh mảng
                        for i in 1..9
                        {
                            for j in 1..9
                            {
                                if arr[i][j]==3
                                {
                                    arr[i][j]=0;
                                }
                            }
                        }
                        
                        if arr[i+1][j]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr[i+k][j]==2 && check==true
                            {
                                if arr[i+k+1][j]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i+l][j]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i-1][j]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr[i-k][j]==2 && check==true
                            {
                                if arr[i-k-1][j]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i-l][j]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr[i][j+k]==2 && check==true
                            {
                                if arr[i][j+k+1]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i][j+l]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr[i][j-k]==2 && check==true
                            {
                                if arr[i][j-k-1]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i][j-l]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i+1][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr[i+k][j+k]==2 && check==true
                            {
                                if arr[i+k+1][j+k+1]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i+l][j+l]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i-1][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr[i-k][j-k]==2 && check==true
                            {
                                if arr[i-k-1][j-k-1]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i-l][j-l]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i+1][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr[i+k][j-k]==2 && check==true
                            {
                                if arr[i+k+1][j-k-1]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i+l][j-l]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i-1][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr[i-k][j+k]==2 && check==true
                            {
                                if arr[i-k-1][j+k+1]==1
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i-l][j+l]=1;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                    }
                }  
            }
        }
    }
    else //đến lượt máy đánh
    {
        game_state.luotchoi=true;
        //check vị trí xem có khả thi ko
        for i in 1..9
        {
            for j in 1..9
            {
                if arr[i][j]==0 && arr[i+1][j]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while i+k<8 && arr[i+k][j]==1 && check==true
                    {
                        if arr[i+k+1][j]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }
                
                if arr[i][j]==0 && arr[i-1][j]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while i-k>1 && arr[i-k][j]==1 && check==true
                    {
                        if arr[i-k-1][j]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i][j+1]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while j+k<8 && arr[i][j+k]==1 && check==true
                    {
                        if arr[i][j+k+1]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i][j-1]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while j-k>1 && arr[i][j-k]==1 && check==true
                    {
                        if arr[i][j-k-1]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i+1][j+1]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while i+k<8 && j+k<8 && arr[i+k][j+k]==1 && check==true
                    {
                        if arr[i+k+1][j+k+1]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i-1][j-1]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while i-k>1 && j-k>1 && arr[i-k][j-k]==1 && check==true
                    {
                        if arr[i-k-1][j-k-1]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i-1][j+1]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while i-k>1 && j+k<8 && arr[i-k][j+k]==1 && check==true
                    {
                        if arr[i-k-1][j+k+1]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }

                if arr[i][j]==0 && arr[i+1][j-1]==1
                {
                    let mut k=1;
                    let mut check=true;
                    while i+k<8 && j-k>1 && arr[i+k][j-k]==1 && check==true
                    {
                        if arr[i+k+1][j-k-1]==2
                        {
                            arr[i][j]=3;
                            check=false;
                            game_state.luotchoi=false;
                        }
                        k+=1;
                    }
                }
            }
        }

    

        //tìm vị trí đánh tối ưu
        if game_state.luotchoi==false
        {   
            let now = Instant::now();

            let mut priority=[[0i32;10];10];
            let mut dem=-999999999;
            for i in 1..9
            {
                for j in 1..9
                {
                    if arr[i][j]==3
                    {
                        let mut arr1=arr;
                        arr1[i][j]=2;

                        if arr1[i+1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr1[i+k][j]==1 && check==true
                            {
                                if arr1[i+k+1][j]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i+l][j]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i-1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr1[i-k][j]==1 && check==true
                            {
                                if arr1[i-k-1][j]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i-l][j]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr1[i][j+k]==1 && check==true
                            {
                                if arr1[i][j+k+1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i][j+l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr1[i][j-k]==1 && check==true
                            {
                                if arr1[i][j-k-1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i][j-l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i+1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr1[i+k][j+k]==1 && check==true
                            {
                                if arr1[i+k+1][j+k+1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i+l][j+l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i-1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr1[i-k][j-k]==1 && check==true
                            {
                                if arr1[i-k-1][j-k-1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i-l][j-l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i+1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr1[i+k][j-k]==1 && check==true
                            {
                                if arr1[i+k+1][j-k-1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i+l][j-l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr1[i-1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr1[i-k][j+k]==1 && check==true
                            {
                                if arr1[i-k-1][j+k+1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr1[i-l][j+l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        //refresh mảng
                        for i1 in 1..9
                        {
                            for j1 in 1..9
                            {
                                if arr1[i1][j1]==3
                                {
                                    arr1[i1][j1]=0;
                                }
                            }
                        }
                        let mut min1=999999999;
                        min1=min(min1,TRY(arr1, true,1,-999999999,999999999));
                        priority[i][j]=min1;
                        dem=max(priority[i][j], dem);
                    }
                }                     
            }
            
            // in ra bảng độ ưu tiên nước đi của máy 
            for i in 1..9
            {
                for j in 1..9
                {
                    print!("{}   ",priority[j][9-i]);
                }
                println!();
            }
            println!("{} *",dem);

            for i in 1..9
            {
                for j in 1..9
                {
                    if arr[i][j]==3 && priority[i][j]==dem && game_state.luotchoi==false
                    {
                        game_state.luotchoi=true;
                        arr[i][j]=2;

                        if arr[i+1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr[i+k][j]==1 && check==true
                            {
                                if arr[i+k+1][j]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i+l][j]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i-1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr[i-k][j]==1 && check==true
                            {
                                if arr[i-k-1][j]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i-l][j]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr[i][j+k]==1 && check==true
                            {
                                if arr[i][j+k+1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i][j+l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr[i][j-k]==1 && check==true
                            {
                                if arr[i][j-k-1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i][j-l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i+1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr[i+k][j+k]==1 && check==true
                            {
                                if arr[i+k+1][j+k+1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i+l][j+l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i-1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr[i-k][j-k]==1 && check==true
                            {
                                if arr[i-k-1][j-k-1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i-l][j-l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i+1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr[i+k][j-k]==1 && check==true
                            {
                                if arr[i+k+1][j-k-1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i+l][j-l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        if arr[i-1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr[i-k][j+k]==1 && check==true
                            {
                                if arr[i-k-1][j+k+1]==2
                                {
                                    for l in 1..k+1
                                    {
                                        arr[i-l][j+l]=2;
                                    }
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        //refresh mảng
                        for i1 in 1..9
                        {
                            for j1 in 1..9
                            {
                                if arr[i1][j1]==3
                                {
                                    arr[i1][j1]=0;
                                }
                            }
                        }
                    }
                }
            }

            // in ra thời gian tính toán nước đi
            let elapsed = now.elapsed();
            println!("Time: {:?}", elapsed);
        }
    }
    
       
    //new game
    for event in engine.collision_events.drain(..) 
    {
        //nếu va chạm không có "point" thì bỏ qua
        if event.pair.0=="point" || event.pair.1=="point"
        {
        
            match event.state 
            {
                CollisionState::Begin => 
                {
                    //đánh dấu những sprite đang va chạm
                    if let Some(sprite) = engine.sprites.get_mut(&event.pair.0)
                    {
                        sprite.mark=true;
                    }
                    if let Some(sprite) = engine.sprites.get_mut(&event.pair.1)
                    {
                        sprite.mark=true;
                    }
                }
                CollisionState::End => 
                {
                    if let Some(sprite) = engine.sprites.get_mut(&event.pair.0)
                    {
                        sprite.mark=false;
                    }
                    if let Some(sprite) = engine.sprites.get_mut(&event.pair.1)
                    {
                        sprite.mark=false;
                    }
                }
            }
        }
    }
    if engine.mouse_state.just_pressed(MouseButton::Left)
    {
        for sprite in engine.sprites.values_mut()
        {
            if sprite.mark==true && sprite.label=="newgame"
            {
                game_state.luotchoi=true;
                for i in 1..9
                {
                    for j in 1..9
                    {
                        arr[i][j]=0;
                    }
                }
                arr[4][4]=2;
                arr[5][4]=1;
                arr[4][5]=1;
                arr[5][5]=2;
            }
        }
    }

    //cập nhật màn hình
    for sprite in engine.sprites.values_mut()
    {
        if sprite.label!="khung" && sprite.label!="point" && sprite.label!="newgame"
        {
            sprite.check=false;
            let i1=(sprite.translation.x+225.0)/50.0;
            let j1=(sprite.translation.y+225.0)/50.0;
            let i=i1 as usize;
            let j=j1 as usize;
            if arr[i][j]==1 && sprite.num==1
            {
                sprite.layer=10.0;
            }else
            if arr[i][j]==2 && sprite.num==2
            {
                sprite.layer=10.0;
            }else
            if arr[i][j]==3 && sprite.num==3
            {
                sprite.layer=10.0;
            }
            else
            {
                sprite.layer=0.0;
            }
        }
    }
    
    //hiển thị số quân đen và trắng mỗi bên
    let mut player=0;
    let mut com=0;
    for i in 1..9
    {
        for j in 1..9
        {
            if arr[i][j]==1
            {
                player+=1;
            }
            if arr[i][j]==2
            {
                com+=1;
            }
        }
    }
    let text1= engine.texts.get_mut("text1").unwrap();
    text1.value=format!("BLACK: {}",com);
    let text2= engine.texts.get_mut("text2").unwrap();
    text2.value=format!("WHITE: {}",player);

}





fn TRY(mut arr:[[i32;10];10],mut turn:bool,depth:i32,mut alpha:i32,mut beta:i32)->i32
{
    let mut endgame=true;
    //check xem có còn nước đi nào ko
    for i in 1..9
    {
        for j in 1..9
        {
            //check quân trắng
            if arr[i][j]==0 && arr[i+1][j]==2
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && arr[i+k][j]==2 && check==true
                {
                    if arr[i+k+1][j]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }
            
            if arr[i][j]==0 && arr[i-1][j]==2
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && arr[i-k][j]==2 && check==true
                {
                    if arr[i-k-1][j]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j+1]==2
            {
                let mut k=1;
                let mut check=true;
                while j+k<8 && arr[i][j+k]==2 && check==true
                {
                    if arr[i][j+k+1]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j-1]==2
            {
                let mut k=1;
                let mut check=true;
                while j-k>1 && arr[i][j-k]==2 && check==true
                {
                    if arr[i][j-k-1]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j+1]==2
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j+k<8 && arr[i+k][j+k]==2 && check==true
                {
                    if arr[i+k+1][j+k+1]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j-1]==2
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j-k>1 && arr[i-k][j-k]==2 && check==true
                {
                    if arr[i-k-1][j-k-1]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j+1]==2
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j+k<8 && arr[i-k][j+k]==2 && check==true
                {
                    if arr[i-k-1][j+k+1]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j-1]==2
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j-k>1 && arr[i+k][j-k]==2 && check==true
                {
                    if arr[i+k+1][j-k-1]==1
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }
            
            //check quân đen
            if arr[i][j]==0 && arr[i+1][j]==1
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && arr[i+k][j]==1 && check==true
                {
                    if arr[i+k+1][j]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }
            
            if arr[i][j]==0 && arr[i-1][j]==1
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && arr[i-k][j]==1 && check==true
                {
                    if arr[i-k-1][j]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j+1]==1
            {
                let mut k=1;
                let mut check=true;
                while j+k<8 && arr[i][j+k]==1 && check==true
                {
                    if arr[i][j+k+1]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i][j-1]==1
            {
                let mut k=1;
                let mut check=true;
                while j-k>1 && arr[i][j-k]==1 && check==true
                {
                    if arr[i][j-k-1]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j+1]==1
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j+k<8 && arr[i+k][j+k]==1 && check==true
                {
                    if arr[i+k+1][j+k+1]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j-1]==1
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j-k>1 && arr[i-k][j-k]==1 && check==true
                {
                    if arr[i-k-1][j-k-1]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i-1][j+1]==1
            {
                let mut k=1;
                let mut check=true;
                while i-k>1 && j+k<8 && arr[i-k][j+k]==1 && check==true
                {
                    if arr[i-k-1][j+k+1]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }

            if arr[i][j]==0 && arr[i+1][j-1]==1
            {
                let mut k=1;
                let mut check=true;
                while i+k<8 && j-k>1 && arr[i+k][j-k]==1 && check==true
                {
                    if arr[i+k+1][j-k-1]==2
                    {
                        check=false;
                        endgame=false;
                    }
                    k+=1;
                }
            }  
        }
    }

    if endgame==true            //hết nước đi
    {
        let mut player=0;
        let mut com=0;
        for i in 1..9
        {
            for j in 1..9
            {
                if arr[i][j]==1
                {
                    player+=1;
                }
                if arr[i][j]==2
                {
                    com+=1;
                }
            }
        }
        if player>com
        {
            return -999999999;
        }else
        if player<com
        {
            return 999999999;
        }else
        {
            return 0;
        }
    }
    else                        //còn nước đi
    {
        if depth<5
        {
            if turn==true           //MIN
            {
                turn=false;
                //check các vị trí khả thi
                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==0 && arr[i+1][j]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr[i+k][j]==2 && check==true
                            {
                                if arr[i+k+1][j]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }
                        
                        if arr[i][j]==0 && arr[i-1][j]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr[i-k][j]==2 && check==true
                            {
                                if arr[i-k-1][j]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr[i][j+k]==2 && check==true
                            {
                                if arr[i][j+k+1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr[i][j-k]==2 && check==true
                            {
                                if arr[i][j-k-1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr[i+k][j+k]==2 && check==true
                            {
                                if arr[i+k+1][j+k+1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr[i-k][j-k]==2 && check==true
                            {
                                if arr[i-k-1][j-k-1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr[i-k][j+k]==2 && check==true
                            {
                                if arr[i-k-1][j+k+1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr[i+k][j-k]==2 && check==true
                            {
                                if arr[i+k+1][j-k-1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=true;
                                }
                                k+=1;
                            }
                        }
                    }
                }

                
                if turn==true
                {
                    let mut min1=999999999;
                    //chọn 1 ví trí để đánh
                    'inter:for i in 1..9
                    {
                        for j in 1..9
                        {   
                            if arr[i][j]==3
                            {
                                let mut arr1=arr;
                                arr1[i][j]=1;
                            
                                //refresh mảng
                                for i1 in 1..9
                                {
                                    for j1 in 1..9
                                    {
                                        if arr1[i1][j1]==3
                                        {
                                            arr1[i1][j1]=0;
                                        }
                                    }
                                }
                                
                                if arr1[i+1][j]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i+k<8 && arr1[i+k][j]==2 && check==true
                                    {
                                        if arr1[i+k+1][j]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i+l][j]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i-1][j]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i-k>1 && arr1[i-k][j]==2 && check==true
                                    {
                                        if arr1[i-k-1][j]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i-l][j]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i][j+1]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while j+k<8 && arr1[i][j+k]==2 && check==true
                                    {
                                        if arr1[i][j+k+1]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i][j+l]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i][j-1]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while j-k>1 && arr1[i][j-k]==2 && check==true
                                    {
                                        if arr1[i][j-k-1]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i][j-l]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i+1][j+1]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i+k<8 && j+k<8 && arr1[i+k][j+k]==2 && check==true
                                    {
                                        if arr1[i+k+1][j+k+1]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i+l][j+l]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i-1][j-1]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i-k>1 && j-k>1 && arr1[i-k][j-k]==2 && check==true
                                    {
                                        if arr1[i-k-1][j-k-1]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i-l][j-l]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i+1][j-1]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i+k<8 && j-k>1 && arr1[i+k][j-k]==2 && check==true
                                    {
                                        if arr1[i+k+1][j-k-1]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i+l][j-l]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i-1][j+1]==2
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i-k>1 && j+k<8 && arr1[i-k][j+k]==2 && check==true
                                    {
                                        if arr1[i-k-1][j+k+1]==1
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i-l][j+l]=1;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                let mut max1=-999999999;
                                max1=max(max1,TRY(arr1, false,depth+1,alpha,beta));
                                min1=min(min1,max1);
                                beta=min(beta, min1);
                                if alpha>=beta
                                {
                                    break 'inter;
                                }
                            }
                        }
                    }
                    return min1;
                }
                else
                {
                    return 999999999;
                }      
            }
            else                    //MAX
            {
                turn=true;
                //check các vị trí khả thi
                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==0 && arr[i+1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr[i+k][j]==1 && check==true
                            {
                                if arr[i+k+1][j]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }
                        
                        if arr[i][j]==0 && arr[i-1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr[i-k][j]==1 && check==true
                            {
                                if arr[i-k-1][j]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr[i][j+k]==1 && check==true
                            {
                                if arr[i][j+k+1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr[i][j-k]==1 && check==true
                            {
                                if arr[i][j-k-1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr[i+k][j+k]==1 && check==true
                            {
                                if arr[i+k+1][j+k+1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr[i-k][j-k]==1 && check==true
                            {
                                if arr[i-k-1][j-k-1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr[i-k][j+k]==1 && check==true
                            {
                                if arr[i-k-1][j+k+1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr[i+k][j-k]==1 && check==true
                            {
                                if arr[i+k+1][j-k-1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                    turn=false;
                                }
                                k+=1;
                            }
                        }
                    }
                }

                if turn==false
                {
                    let mut max1=-999999999;
                    //chọn 1 ví trí để đánh
                    'outer:for i in 1..9
                    {
                        for j in 1..9
                        {   
                            if arr[i][j]==3
                            {
                                let mut arr1=arr;
                                arr1[i][j]=2;
                            
                                //refresh mảng
                                for i1 in 1..9
                                {
                                    for j1 in 1..9
                                    {
                                        if arr1[i1][j1]==3
                                        {
                                            arr1[i1][j1]=0;
                                        }
                                    }
                                }
                                
                                if arr1[i+1][j]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i+k<8 && arr1[i+k][j]==1 && check==true
                                    {
                                        if arr1[i+k+1][j]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i+l][j]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i-1][j]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i-k>1 && arr1[i-k][j]==1 && check==true
                                    {
                                        if arr1[i-k-1][j]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i-l][j]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i][j+1]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while j+k<8 && arr1[i][j+k]==1 && check==true
                                    {
                                        if arr1[i][j+k+1]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i][j+l]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i][j-1]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while j-k>1 && arr1[i][j-k]==1 && check==true
                                    {
                                        if arr1[i][j-k-1]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i][j-l]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i+1][j+1]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i+k<8 && j+k<8 && arr1[i+k][j+k]==1 && check==true
                                    {
                                        if arr1[i+k+1][j+k+1]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i+l][j+l]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i-1][j-1]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i-k>1 && j-k>1 && arr1[i-k][j-k]==1 && check==true
                                    {
                                        if arr1[i-k-1][j-k-1]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i-l][j-l]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i+1][j-1]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i+k<8 && j-k>1 && arr1[i+k][j-k]==1 && check==true
                                    {
                                        if arr1[i+k+1][j-k-1]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i+l][j-l]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                if arr1[i-1][j+1]==1
                                {
                                    let mut k=1;
                                    let mut check=true;
                                    while i-k>1 && j+k<8 && arr1[i-k][j+k]==1 && check==true
                                    {
                                        if arr1[i-k-1][j+k+1]==2
                                        {
                                            for l in 1..k+1
                                            {
                                                arr1[i-l][j+l]=2;
                                            }
                                            check=false;
                                        }
                                        k+=1;
                                    }
                                }
                                let mut min1=999999999;
                                min1=min(min1,TRY(arr1, true,depth+1,alpha,beta));
                                max1=max(max1,min1);
                                alpha=max(alpha,max1);
                                if alpha>=beta
                                {
                                    break 'outer;
                                }
                            }
                        }
                    }
                    return max1;
                }
                else
                {
                    return -999999999;
                }
            }
        }
        else
        {
            //điểm vị trí
            let mut priority = [[0i32;10];10];
            for i in 1..9
            {
                for j in 1..9
                {
                    priority[i][j]=1;
                }
            }
            priority[1][1]=1000;priority[2][2]=-20;
            priority[1][8]=1000;priority[7][7]=-20;
            priority[8][1]=1000;priority[2][7]=-20;
            priority[8][8]=1000;priority[7][2]=-20;
    
            priority[1][2]=20;priority[1][3]=20;priority[1][4]=20;
            priority[2][1]=20;priority[1][6]=20;priority[1][5]=20;
            priority[1][7]=20;priority[3][1]=20;priority[4][1]=20;
            priority[7][1]=20;priority[6][1]=20;priority[5][1]=20;
            priority[7][8]=20;priority[8][3]=20;priority[8][4]=20;
            priority[8][7]=20;priority[8][6]=20;priority[8][5]=20;
            priority[2][8]=20;priority[3][8]=20;priority[4][8]=20;
            priority[8][2]=20;priority[6][8]=20;priority[5][8]=20;

            priority[2][3]=-1;priority[7][3]=-1;priority[3][2]=-1;priority[3][7]=-1;
            priority[2][4]=-1;priority[7][4]=-1;priority[4][2]=-1;priority[4][7]=-1;
            priority[2][5]=-1;priority[7][5]=-1;priority[5][2]=-1;priority[5][7]=-1;
            priority[2][6]=-1;priority[7][6]=-1;priority[6][2]=-1;priority[6][7]=-1;

            let mut point=0;
            for i in 1..9
            {
                for j in 1..9
                {
                    if arr[i][j]==1
                    {
                        point-=priority[i][j];
                    }
                    if arr[i][j]==2
                    {
                        point+=priority[i][j];
                    }
                }
            }
            let mut dem=0;
            //điểm độ cơ động(lớn ở đầu game và bằng 0 vào cuối game)
            for i in 1..9
            {
                for j in 1..9
                {
                    if arr[i][j]==0
                    {
                        dem+=1;
                    }
                }
            }
            //giai đoạn đầu 
            if dem>10
            {
                //đếm số nước đi quân trắng có thể đi
                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==0 && arr[i+1][j]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr[i+k][j]==2 && check==true
                            {
                                if arr[i+k+1][j]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        
                        if arr[i][j]==0 && arr[i-1][j]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr[i-k][j]==2 && check==true
                            {
                                if arr[i-k-1][j]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr[i][j+k]==2 && check==true
                            {
                                if arr[i][j+k+1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr[i][j-k]==2 && check==true
                            {
                                if arr[i][j-k-1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr[i+k][j+k]==2 && check==true
                            {
                                if arr[i+k+1][j+k+1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr[i-k][j-k]==2 && check==true
                            {
                                if arr[i-k-1][j-k-1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j+1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr[i-k][j+k]==2 && check==true
                            {
                                if arr[i-k-1][j+k+1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j-1]==2
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr[i+k][j-k]==2 && check==true
                            {
                                if arr[i+k+1][j-k-1]==1
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                    }
                }

                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==3
                        {
                            point-=10;
                            arr[i][j]=0;
                        }
                    }
                }

                //đếm số nước đi quân đen có thể đi
                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==0 && arr[i+1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && arr[i+k][j]==1 && check==true
                            {
                                if arr[i+k+1][j]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                        
                        if arr[i][j]==0 && arr[i-1][j]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && arr[i-k][j]==1 && check==true
                            {
                                if arr[i-k-1][j]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j+k<8 && arr[i][j+k]==1 && check==true
                            {
                                if arr[i][j+k+1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while j-k>1 && arr[i][j-k]==1 && check==true
                            {
                                if arr[i][j-k-1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j+k<8 && arr[i+k][j+k]==1 && check==true
                            {
                                if arr[i+k+1][j+k+1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j-k>1 && arr[i-k][j-k]==1 && check==true
                            {
                                if arr[i-k-1][j-k-1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i-1][j+1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i-k>1 && j+k<8 && arr[i-k][j+k]==1 && check==true
                            {
                                if arr[i-k-1][j+k+1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }

                        if arr[i][j]==0 && arr[i+1][j-1]==1
                        {
                            let mut k=1;
                            let mut check=true;
                            while i+k<8 && j-k>1 && arr[i+k][j-k]==1 && check==true
                            {
                                if arr[i+k+1][j-k-1]==2
                                {
                                    arr[i][j]=3;
                                    check=false;
                                }
                                k+=1;
                            }
                        }
                    }
                }

                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==3
                        {
                            point+=10;
                            arr[i][j]=0;
                        }
                    }
                }
            }
            else    //giai đoạn cuối ván cờ
            {
                //giai đoạn sau đếm số quân cờ 2 bên
                for i in 1..9
                {
                    for j in 1..9
                    {
                        if arr[i][j]==1
                        {
                            point-=100;
                        }
                        if arr[i][j]==2
                        {
                            point+=100;
                        }
                    }
                }
            }

            return point;
        }
    }


}


fn max(a:i32,b:i32)->i32
{
    if a>b
    {
        return a;
    }
    else 
    {
        return b;
    }
}
fn min(a:i32,b:i32)->i32
{
    if a<b
    {
        return a;
    }
    else 
    {
        return b;
    }
}