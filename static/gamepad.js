let gamepad;
let connectionStatus;
let previousTime = 0;

let gamepad_connection = false;

setInterval(update, 500);

window.addEventListener("gamepadconnected", function (e) {
    gamepad = navigator.getGamepads()[e.gamepad.index];
    connectionStatus = document.getElementById("connectionStatus");
    console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
        e.gamepad.index, e.gamepad.id,
        e.gamepad.buttons.length, e.gamepad.axes.length);

    connectionStatus.innerHTML = "Connected";
    connectionStatus.style.color = "black";
    gamepad_connection = true;
});

window.addEventListener("gamepaddisconnected", function () {
    console.log("Gamepad disconnected");
    connectionStatus.innerHTML = "Disconnected";
    connectionStatus.style.color = "red";
    gamepad_connection = false;
});

function update() {
    if (gamepad_connection) {
        render();
    }
}

function render() {
    let leftJoyXAxis = gamepad.axes[0];
    let leftJoyYAxis = gamepad.axes[1];
    let rightJoyXAxis = gamepad.axes[3];
    let rightJoyYAxis = gamepad.axes[4];

    let aButton = gamepad.buttons[0].pressed;
    let bButton = gamepad.buttons[1].pressed;
    let xButton = gamepad.buttons[2].pressed;
    let yButton = gamepad.buttons[3].pressed;
    let lbButton = gamepad.buttons[4].pressed;
    let rbButton = gamepad.buttons[5].pressed;
    let backButton = gamepad.buttons[6].pressed;
    let startButton = gamepad.buttons[7].pressed;
    let homeButton = gamepad.buttons[8].pressed;
    let leftJoyButton = gamepad.buttons[9].pressed;
    let rightJoyButton = gamepad.buttons[10].pressed;

    draw_axis("movement-axis", leftJoyXAxis * 45 + 50, leftJoyYAxis * 45 + 50);
    draw_axis("camera-axis", rightJoyXAxis * 45 + 50, rightJoyYAxis * 45 + 50);

    console.log("(" + leftJoyYAxis + ", " + rightJoyYAxis + ")");

    draw_button("a", aButton);
    draw_button("b", bButton);
    draw_button("x", xButton);
    draw_button("y", yButton);

    draw_button("lb", lbButton);
    draw_button("rb", rbButton);

    draw_button("back", backButton);
    draw_button("start", startButton);
    draw_button("ljoy", leftJoyButton);
    draw_button("rjoy", rightJoyButton);

    drive(-leftJoyYAxis, -rightJoyYAxis);
}

function draw_axis(axis, x, y) {
    let canvas = document.getElementById(axis);
    let width = canvas.width;
    let height = canvas.height;
    let context = canvas.getContext("2d");
    context.clearRect(0, 0, width, height);
    context.beginPath();
    context.moveTo(x - 5, y);
    context.lineTo(x + 5, y);
    context.moveTo(x, y - 5);
    context.lineTo(x, y + 5);
    context.stroke();
}

function draw_button(name, state) {
    let data = document.getElementById(name);
    if (state) {
        data.style.backgroundColor = "red";
    } else {
        data.style.backgroundColor = "white";
    }
}