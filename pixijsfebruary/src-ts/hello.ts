declare const PIXI: any;

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
    sprite_green.position.set(30, 30);
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
        buttonClicked('H');
    });

    const buttonJ = document.getElementById('btn-j');
    buttonJ?.addEventListener('click', () => {
        buttonClicked('J');
    });

    const buttonK = document.getElementById('btn-k');
    buttonK?.addEventListener('click', () => {
        buttonClicked('K');
    });

    const buttonL = document.getElementById('btn-l');
    buttonL?.addEventListener('click', () => {
        buttonClicked('L');
    });

    window.addEventListener('keydown', (event) => {
        switch (event.key.toLowerCase()) {
            case 'h':
                buttonClicked('H');
                break;
            case 'j':
                buttonClicked('J');
                break;
            case 'k':
                buttonClicked('K');
                break;
            case 'l':
                buttonClicked('L');
                break;
        }
    });

    function buttonClicked(btn: string) {
        switch (btn) {
            case 'H':
                sprite_green.x -= 10;
                break;
            case 'J':
                sprite_green.y += 10;
                break;
            case 'K':
                sprite_green.y -= 10;
                break;
            case 'L':
                sprite_green.x += 10;
                break;
        }
        console.log(`[ ${btn} ${sprite_green.x} ${sprite_green.y} ] ${crypto.randomUUID()}`);
    }
})();