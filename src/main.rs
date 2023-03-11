use le_petit_lapin::keys::*;
use le_petit_lapin::layouts::*;
use le_petit_lapin::rules::*;
use le_petit_lapin::*;
use std::env;

#[rustfmt::skip]
fn main() {
    let mut lapin = Lapin::connect();

    const MODKEY: &str = "Super";
    const TERMINAL: &str = "alacritty";

    let mut keybinds = KeybindSet::new();
    keybinds.bindall(vec![
        // workspace keys
        (&[MODKEY], "1", lazy! {wm, wm.goto_workspace(0)}),
        (&[MODKEY], "2", lazy! {wm, wm.goto_workspace(1)}),
        (&[MODKEY], "3", lazy! {wm, wm.goto_workspace(2)}),
        (&[MODKEY], "4", lazy! {wm, wm.goto_workspace(3)}),
        (&[MODKEY], "5", lazy! {wm, wm.goto_workspace(4)}),
        (&[MODKEY], "6", lazy! {wm, wm.goto_workspace(5)}),
        (&[MODKEY], "7", lazy! {wm, wm.goto_workspace(6)}),
        (&[MODKEY], "8", lazy! {wm, wm.goto_workspace(7)}),
        (&[MODKEY], "9", lazy! {wm, wm.goto_workspace(8)}),
        (&[MODKEY, "Shift"], "1", lazy! {wm, wm.send_window_to_workspace(0)}),
        (&[MODKEY, "Shift"], "2", lazy! {wm, wm.send_window_to_workspace(1)}),
        (&[MODKEY, "Shift"], "3", lazy! {wm, wm.send_window_to_workspace(2)}),
        (&[MODKEY, "Shift"], "4", lazy! {wm, wm.send_window_to_workspace(3)}),
        (&[MODKEY, "Shift"], "5", lazy! {wm, wm.send_window_to_workspace(4)}),
        (&[MODKEY, "Shift"], "6", lazy! {wm, wm.send_window_to_workspace(5)}),
        (&[MODKEY, "Shift"], "7", lazy! {wm, wm.send_window_to_workspace(6)}),
        (&[MODKEY, "Shift"], "8", lazy! {wm, wm.send_window_to_workspace(7)}),
        (&[MODKEY, "Shift"], "9", lazy! {wm, wm.send_window_to_workspace(8)}),
        // kill focus
        (&[MODKEY], "w", lazy! {wm, wm.killfocused()}),
        // change focus
        (&[MODKEY], "j", lazy! {wm, wm.nextwin()}),
        (&[MODKEY], "k", lazy! {wm, wm.prevwin()}),
        // change layout
        (&[MODKEY], "space", lazy! {wm, wm.next_layout()}),
        (&[MODKEY, "Shift"], "space", lazy! {wm, wm.prev_layout()}),
        // swap slaves
        (&[MODKEY, "Shift"], "k", lazy! {wm, wm.swap_with_prev_slave()}),
        (&[MODKEY, "Shift"], "j", lazy! {wm, wm.swap_with_next_slave()}),
        // change master
        (&[MODKEY, "Shift"], "Return", lazy! {wm, wm.change_master()}),
        // toggle ool
        (&[MODKEY, "Shift"], "t", lazy! {wm, wm.toggle_ool()}),
        // fullscreen
        (&[MODKEY, "Shift"], "f", lazy! {wm, wm.fullscreen()}),
	// outer gaps
        (&[MODKEY, "Shift"], "b", lazy! {wm, wm.toggle_reserved_space()}),
        // change focused screen (monitor)
        (&[MODKEY], "y", lazy! {wm, wm.prev_screen()}),
        (&[MODKEY], "u", lazy! {wm, wm.next_screen()}),
        // change focused window screen
        (&[MODKEY, "Shift"], "y", lazy! {wm, wm.send_window_to_prev_screen()}),
        (&[MODKEY, "Shift"], "u", lazy! {wm, wm.send_window_to_next_screen()}),

        // applications
        (&[MODKEY], "Return", lazy! {Lapin::spawn(TERMINAL)}),
        (&[MODKEY], "m", lazy! {Lapin::spawn(&format!("{TERMINAL} --class=floating -e ncmpcpp"))}),
        (&[MODKEY], "s", lazy! {Lapin::spawn(&format!("{TERMINAL} --class=floating -e pulsemixer"))}),
        (&[MODKEY], "c", lazy! {Lapin::spawn(&format!("{TERMINAL} --class=floating -e julia"))}),
        (&[MODKEY], "t", lazy! {Lapin::spawn(&format!("{TERMINAL} --class=floating -e htop"))}),
        (&[MODKEY], "n", lazy! {Lapin::spawn("chromium")}),
        (&[MODKEY], "p", lazy! {Lapin::spawn("screenshotter")}),
        (&[MODKEY, "Shift"], "p", lazy! {Lapin::spawn("screenshotter select")}),
        (&[MODKEY], "e", lazy! {Lapin::spawn("emacsclient -c -a emacs")}),
	    // dmenu/rofi
        (&[MODKEY], "a", lazy! {Lapin::spawn("rofi -show run")}),
        (&[MODKEY], "q", lazy! {Lapin::spawn("dmenu_shutdown")}),
        (&[MODKEY, "Ctrl"], "p", lazy! {Lapin::spawn("passmenu --type")}),
        (&[MODKEY], "b", lazy! {Lapin::spawn("dmenu_web")}),
        (&[MODKEY, "Ctrl"], "b", lazy! {Lapin::spawn("dmenu_bluetooth")}),
        (&[MODKEY, "Ctrl"], "d", lazy! {Lapin::spawn("monitors")}),
	    // audio
        (&["Meta", "Ctrl"], "k", lazy! {Lapin::spawn("pulsemixer --change-volume +1")}),
        (&["Meta", "Ctrl"], "j", lazy! {Lapin::spawn("pulsemixer --change-volume -1")}),
        (&["Meta", "Ctrl"], "m", lazy! {Lapin::spawn("pulsemixer --toggle-mute")}),
        (&["Meta", "Ctrl"], "p", lazy! {Lapin::spawn("mpc toggle")}),
        (&["Meta", "Ctrl"], "l", lazy! {Lapin::spawn("mpc next")}),
        (&["Meta", "Ctrl"], "h", lazy! {Lapin::spawn("mpc prev")}),

        (&[MODKEY, "Ctrl"], "c", lazy! {Lapin::spawn("dunstctl close")}),
    ]);

    lapin.config.mouse_mod = &[MODKEY];
    lapin.config.mouse_raises_window = false;

    lapin.config.border_color = 0xff000000;
    lapin.config.border_color_focus = 0xff00bb00;
    lapin.config.border_width = 4;
    lapin.config.reserved_space = (30, 0, 0, 0);

    let tile = Tiling {
	    name: "tile",
	    borders: 4,
	    master_factor: 1.0 / 2.0,
	    gaps: 10,
    };
    let max = Maximized {
	    name: "max",
	    borders: 4,
	    gaps: 0,
    };
    let float = Floating {
	    name: "float",
	    borders: 4,
    };
    lapin.config.layouts = layouts![tile, max, float];

    lapin.config.rules = vec![
	    rule!(class "Gimp" => Apply::Float),
	    rule!(class "QjackCtl" => Apply::Float),
	    rule!(class "Guitarix" => Apply::Float),
	    rule!(class "floating" => Apply::Float),
    ];

    let mut callback = lazy! {{
	    let home = env!("HOME");
	    Lapin::spawn("picom");
	    Lapin::spawn("polybar");
	    Lapin::spawn("luabatmon");
	    Lapin::spawn("/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1");
	    Lapin::spawn("unclutter --timeout --jitter --start-hidden");
	    Lapin::spawn(&format!("feh --no-fehbg --bg-fill {home}/pics/wallpapers/share/cyberpunk/nord-3.png"));
    }};

    lapin.init(&mut keybinds, Some(&mut callback));
}
