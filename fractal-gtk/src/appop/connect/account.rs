use crate::app::AppRuntime;
use crate::ui::UI;
use gio::ActionMapExt;
use glib::clone;
use gtk::prelude::*;

use crate::actions::{AccountSettings, StateExt};

pub fn connect(ui: &UI, app_runtime: AppRuntime) {
    let builder = &ui.builder;
    let cancel_password = builder
        .get_object::<gtk::Button>("password-dialog-cancel")
        .expect("Can't find password-dialog-cancel in ui file.");
    let confirm_password = builder
        .get_object::<gtk::Button>("password-dialog-apply")
        .expect("Can't find password-dialog-apply in ui file.");
    let password_dialog = builder
        .get_object::<gtk::Dialog>("password_dialog")
        .expect("Can't find password_dialog in ui file.");
    let avatar_btn = builder
        .get_object::<gtk::Button>("account_settings_avatar_button")
        .expect("Can't find account_settings_avatar_button in ui file.");
    let name_entry = builder
        .get_object::<gtk::Entry>("account_settings_name")
        .expect("Can't find account_settings_name in ui file.");
    let name_btn = builder
        .get_object::<gtk::Button>("account_settings_name_button")
        .expect("Can't find account_settings_name_button in ui file.");
    let password_btn = builder
        .get_object::<gtk::Button>("account_settings_password")
        .expect("Can't find account_settings_password in ui file.");
    let old_password = builder
        .get_object::<gtk::Entry>("password-dialog-old-entry")
        .expect("Can't find password-dialog-old-entry in ui file.");
    let new_password = builder
        .get_object::<gtk::Entry>("password-dialog-entry")
        .expect("Can't find password-dialog-entry in ui file.");
    let verify_password = builder
        .get_object::<gtk::Entry>("password-dialog-verify-entry")
        .expect("Can't find password-dialog-verify-entry in ui file.");
    let destruction_entry = builder
        .get_object::<gtk::Entry>("account_settings_delete_password_confirm")
        .expect("Can't find account_settings_delete_password_confirm in ui file.");
    let destruction_btn = builder
        .get_object::<gtk::Button>("account_settings_delete_btn")
        .expect("Can't find account_settings_delete_btn in ui file.");

    let window = ui.main_window.upcast_ref::<gtk::Window>();
    let actions = AccountSettings::new(&window, app_runtime.clone());
    let container = builder
        .get_object::<gtk::Box>("account_settings_box")
        .expect("Can't find account_settings_box in ui file.");
    container.insert_action_group("user-settings", Some(&actions));

    /* Body */
    if let Some(action) = actions.lookup_action("change-avatar") {
        action.bind_button_state(&avatar_btn);
        avatar_btn.set_action_name(Some("user-settings.change-avatar"));
        let avatar_spinner = builder
            .get_object::<gtk::Spinner>("account_settings_avatar_spinner")
            .expect("Can't find account_settings_avatar_spinner in ui file.");
        avatar_btn.connect_property_sensitive_notify(
            clone!(@weak avatar_spinner as spinner => move |w| {
                if w.get_sensitive() {
                    spinner.hide();
                    spinner.stop();
                } else {
                    spinner.start();
                    spinner.show();
                }
            }),
        );
    }

    name_entry.connect_property_text_notify(
        clone!(@strong app_runtime, @strong name_btn as button => move |w| {
            app_runtime.update_state_with(clone!(@strong w, @strong button => move |state| {
                let username = w.get_text();
                if !username.is_empty()
                    && state
                        .login_data
                        .as_ref()
                        .and_then(|login_data| login_data.username.as_ref())
                        .filter(|u| **u != username)
                        .is_some()
                {
                    button.show();
                    return;
                }
                button.hide();
            }));
        }),
    );

    let button = name_btn.clone();
    name_entry.connect_activate(move |_w| {
        let _ = button.emit("clicked", &[]);
    });

    name_btn.connect_clicked(clone!(@strong app_runtime => move |_w| {
        app_runtime.update_state_with(|state| state.update_username_account_settings());
    }));

    /*
    fn update_password_strength(builder: &gtk::Builder) {
    let bar = builder
    .get_object::<gtk::LevelBar>("password-dialog-strength-indicator")
    .expect("Can't find password-dialog-strength-indicator in ui file.");
    let label = builder
    .get_object::<gtk::Label>("password-dialog-hint")
    .expect("Can't find password-dialog-hint in ui file.");
    let strength_level = 10f64;
    bar.set_value(strength_level);
    label.set_label("text");
    }
    */

    fn validate_password_input(builder: &gtk::Builder) {
        let hint = builder
            .get_object::<gtk::Label>("password-dialog-verify-hint")
            .expect("Can't find password-dialog-verify-hint in ui file.");
        let confirm_password = builder
            .get_object::<gtk::Button>("password-dialog-apply")
            .expect("Can't find password-dialog-apply in ui file.");
        let old = builder
            .get_object::<gtk::Entry>("password-dialog-old-entry")
            .expect("Can't find password-dialog-old-entry in ui file.");
        let new = builder
            .get_object::<gtk::Entry>("password-dialog-entry")
            .expect("Can't find password-dialog-entry in ui file.");
        let verify = builder
            .get_object::<gtk::Entry>("password-dialog-verify-entry")
            .expect("Can't find password-dialog-verify-entry in ui file.");

        let mut empty = true;
        let mut matching = true;
        let old_p = old.get_text();
        let new_p = new.get_text();
        let verify_p = verify.get_text();

        if new_p != verify_p {
            matching = false;
        }
        if !new_p.is_empty() && !verify_p.is_empty() && !old_p.is_empty() {
            empty = false;
        }

        if matching {
            hint.hide();
        } else {
            hint.show();
        }

        confirm_password.set_sensitive(matching && !empty);
    }

    /* Passsword dialog */
    password_btn.connect_clicked(clone!(@strong app_runtime => move |_| {
        app_runtime.update_state_with(|state| state.show_password_dialog());
    }));

    password_dialog.connect_delete_event(clone!(@strong app_runtime => move |_, _| {
        app_runtime.update_state_with(|state| state.close_password_dialog());
        glib::signal::Inhibit(true)
    }));

    /* Headerbar */
    cancel_password.connect_clicked(clone!(@strong app_runtime => move |_| {
        app_runtime.update_state_with(|state| state.close_password_dialog());
    }));

    confirm_password.connect_clicked(clone!(@strong app_runtime => move |_| {
        app_runtime.update_state_with(|state| {
            state.set_new_password();
            state.close_password_dialog();
        });
    }));

    /* Body */
    verify_password.connect_property_text_notify(clone!(@strong builder => move |_| {
        validate_password_input(&builder.clone());
    }));
    new_password.connect_property_text_notify(clone!(@strong builder => move |_| {
        validate_password_input(&builder.clone());
    }));
    old_password.connect_property_text_notify(clone!(@strong builder => move |_| {
        validate_password_input(&builder)
    }));

    destruction_entry.connect_property_text_notify(clone!(@strong destruction_btn => move |w| {
        if !w.get_text().is_empty() {
            destruction_btn.set_sensitive(true);
            return;
        }
        destruction_btn.set_sensitive(false);
    }));

    destruction_btn.connect_clicked(move |_| {
        app_runtime.update_state_with(|state| state.account_destruction());
    });
}
