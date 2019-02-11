function drive_from_form() {
    let left = $("#left-drive").val();
    let right = $("#right-drive").val();
    drive(left, right)
}

function get_state() {
    fetch("/robot/state", {method: "GET"})
        .then(function (value) {
            // TODO find a better way to display state.
            $("#state-view").text(JSON.stringify(value))

        })
        .catch(function (reason) {
            alert(reason);
        });
}

function kill() {
    let url = "/robot/kill";
    post_command(url);
}

function revive() {
    let url = "/robot/revive";
    post_command(url);
}

function switch_to_drive() {
    let url = "/robot/modes/drive";
    post_command(url);
}

function switch_to_dump() {
    let url = "/robot/modes/dump";
    post_command(url);
}

function switch_to_dig() {
    let url = "/robot/modes/dig";
    post_command(url);
}

function brake() {
    let url = "/robot/drive_train/brake";
    post_command(url);
}

function dig() {
    post_command("/robot/intake/digger/dig");
}

function stop_digging() {
    post_command("/robot/intake/digger/stop");
}

function raise_actuators() {
    post_command("/robot/intake/rails/raise");
}

function lower_actuators() {
    post_command("/robot/intake/rails/lower");
}

function stop_actuators() {
    post_command("/robot/intake/rails/stop");
}

function drive(left, right) {
    let url = "/robot/drive_train/drive/" + left + "/" + right;
    post_command(url);
}

function post_command(url) {
    fetch(url, {method: "POST"})
        .then(function (value) {
            // I'll be damned if I know what to do here.
        })
        .catch(function (reason) {
            alert("Malformed request!");
        });
}