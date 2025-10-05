#[macro_export]
macro_rules! get_session {
    ($session_id:expr, $error_message:expr) => {{
        match Sessions::share().get(&$session_id) {
            None => return HttpResponseHelper::forbidden()
                .error($error_message)
                .build(),
            Some(session) => session,
        }
    }};
}

#[macro_export]
macro_rules! get_group_controller {
    ($session:expr) => {{
        let group_controller = unsafe {
            if $session.group_controller.is_null() {
                $session.group_controller = pocket_group_controller_new($session.pocket);
                if $session.group_controller.is_null() {
                    return HttpResponseHelper::internal_server_error()
                        .error("Group controller null")
                        .build();
                }
                pocket_group_controller_init($session.group_controller);
            }

            $session.group_controller
        };

        group_controller
    }};
}

#[macro_export]
macro_rules! get_group_field_controller {
    ($session:expr) => {{
        let group_field_controller = unsafe {
            if $session.group_field_controller.is_null() {
                $session.group_field_controller = pocket_group_field_controller_new($session.pocket);
                if $session.group_field_controller.is_null() {
                    return HttpResponseHelper::internal_server_error()
                        .error("Group controller null")
                        .build();
                }
                pocket_group_field_controller_init($session.group_field_controller);
            }

            $session.group_field_controller
        };

        group_field_controller
    }};
}

#[macro_export]
macro_rules! get_field_controller {
    ($session:expr) => {{
        let field_controller = unsafe {
            if $session.field_controller.is_null() {
                $session.field_controller = pocket_field_controller_new($session.pocket);

                if $session.field_controller.is_null() {
                    return HttpResponseHelper::internal_server_error()
                        .error("Field controller null")
                        .build();
                }

                pocket_field_controller_init($session.field_controller);
            }

            $session.field_controller
        };

        field_controller
    }};
}

#[macro_export]
macro_rules! perform_timestamp_last_update {
    ($session:ident) => {
        $session.update_timestamp_last_update();

        Sessions::share().remove(&$session.session_id, false);

        Sessions::share().add($session.clone());
    };
}