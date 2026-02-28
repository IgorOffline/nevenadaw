declare const PIXI: any;

enum FebruaryLogLevel {
    Trace = 0,
    Info = 1,
}

const logLevel: FebruaryLogLevel = FebruaryLogLevel.Trace;
const buttonMovementOffset = 10;

(async () => {
    const div = document.getElementById('pixijsfebruary');
    if (!div) return;

    const app = new PIXI.Application();
    await app.init({
        background: '#455A64',
        resizeTo: div
    });
    const g = new PIXI.Graphics();
    for (let ix = 0; ix < 69; ix++) {
        for (let jy = 0; jy < 69; jy++) {
            g.rect(-100 + ix * 30, -100 + jy * 30, 10, 10);
            g.fill(0x757575);
            g.stroke({color: 0x212121, width: 1.2});
        }
    }
    app.stage.addChild(g);
    div.appendChild(app.canvas);
    const texture_green = await PIXI.Assets.load('alice1a_green.png');
    const sprite_green = new PIXI.Sprite(texture_green);
    app.stage.addChild(sprite_green);
    sprite_green.position.set(50, 50);
    sprite_green.scale.set(0.15);

    const texture_blue = await PIXI.Assets.load('alice1a_blue.png');
    const sprite_blue = new PIXI.Sprite(texture_blue);
    app.stage.addChild(sprite_blue);
    sprite_blue.position.set(50, 300);
    sprite_blue.scale.set(0.15);

    const texture_red = await PIXI.Assets.load('alice1a_red.png');
    const sprite_red = new PIXI.Sprite(texture_red);
    app.stage.addChild(sprite_red);
    sprite_red.position.set(300, 300);
    sprite_red.scale.set(0.15);

    app.ticker.add(() => {
        //
    })

    const buttonH = document.getElementById('btn-h');
    buttonH?.addEventListener('click', () => {
        buttonClicked('H', buttonMovementOffset);
    });

    const buttonJ = document.getElementById('btn-j');
    buttonJ?.addEventListener('click', () => {
        buttonClicked('J', buttonMovementOffset);
    });

    const buttonK = document.getElementById('btn-k');
    buttonK?.addEventListener('click', () => {
        buttonClicked('K', buttonMovementOffset);
    });

    const buttonL = document.getElementById('btn-l');
    buttonL?.addEventListener('click', () => {
        buttonClicked('L', buttonMovementOffset);
    });

    window.addEventListener('keydown', (event) => {
        switch (event.key.toLowerCase()) {
            case 'h':
                buttonClicked('H', buttonMovementOffset);
                break;
            case 'j':
                buttonClicked('J', buttonMovementOffset);
                break;
            case 'k':
                buttonClicked('K', buttonMovementOffset);
                break;
            case 'l':
                buttonClicked('L', buttonMovementOffset);
                break;
        }
    });

    function buttonClicked(btn: string, movementOffset: number) {
        switch (btn) {
            case 'H':
                sprite_green.x -= movementOffset;
                break;
            case 'J':
                sprite_green.y += movementOffset;
                break;
            case 'K':
                sprite_green.y -= movementOffset;
                break;
            case 'L':
                sprite_green.x += movementOffset;
                break;
        }

        const message = `[ ${btn} ${sprite_green.x} ${sprite_green.y} ] ${crypto.randomUUID()}`;
        logInfo(message);
    }

    function logInfo(message: string) {
        if (logLevel === FebruaryLogLevel.Info) {
            console.log(message);
        }
    }
})();