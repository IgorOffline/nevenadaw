declare const PIXI: any;

class FebruaryButton {
    physicalLower: string;
    physicalUpper: string;
    visualId: string;

    constructor(physicalLower: string, physicalUpper: string) {
        this.physicalLower = physicalLower;
        this.physicalUpper = physicalUpper;
        this.visualId = `btn-${physicalLower}`;
    }
}

enum FebruaryLogLevel {
    Trace = 0,
    Info = 1,
}

export const Regina = {
    logLevel: FebruaryLogLevel.Trace as FebruaryLogLevel,
    buttons: {
        H: new FebruaryButton('h', 'H'),
        J: new FebruaryButton('j', 'J'),
        K: new FebruaryButton('k', 'K'),
        L: new FebruaryButton('l', 'L'),
    },
    buttonMovementOffset: 10,
    events: {
        click: 'click',
        keydown: 'keydown',
    },
    font: {
        name: 'FebruaryTerminalRegular',
        url: 'https://igordurbek.b-cdn.net/IosevkaTerm-Regular.ttf',
        size: 72,
    },
    textures: {
        green: 'alice1a_green.png',
        blue: 'alice1a_blue.png',
        red: 'alice1a_red.png',
    },
    textureDefaultTopLeft: 50,
    textureDefaultBottomRight: 300,
    textureScale: 0.15,
} as const;

async function loadMyFont() {
    const font = new FontFace(
        Regina.font.name,
        `url(${Regina.font.url})`
    );

    const loaded = await font.load();
    (document.fonts as unknown as FontFaceSet).add(loaded);

    await document.fonts.load(
        `${Regina.font.size}px "${Regina.font.name}"`
    );

    console.log(`Font loaded: ${Regina.font.name}`);
}

(async () => {
    const div = document.getElementById('pixijsfebruary');
    if (!div) return;

    await loadMyFont();

    const app = new PIXI.Application();
    await app.init({
        background: '#455A64',
        resizeTo: div,
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

    const texture_green = await PIXI.Assets.load(Regina.textures.green);
    const sprite_green = new PIXI.Sprite(texture_green);
    sprite_green.position.set(Regina.textureDefaultTopLeft, Regina.textureDefaultTopLeft);
    sprite_green.scale.set(Regina.textureScale);
    app.stage.addChild(sprite_green);

    const texture_blue = await PIXI.Assets.load(Regina.textures.blue);
    const sprite_blue = new PIXI.Sprite(texture_blue);
    sprite_blue.position.set(Regina.textureDefaultTopLeft, Regina.textureDefaultBottomRight);
    sprite_blue.scale.set(Regina.textureScale);
    app.stage.addChild(sprite_blue);

    const texture_red = await PIXI.Assets.load(Regina.textures.red);
    const sprite_red = new PIXI.Sprite(texture_red);
    sprite_red.position.set(Regina.textureDefaultBottomRight, Regina.textureDefaultBottomRight);
    sprite_red.scale.set(Regina.textureScale);
    app.stage.addChild(sprite_red);

    const label = new PIXI.Text({
        text: '123456789',
        style: {
            fontFamily: Regina.font.name,
            fontSize: Regina.font.size,
            fill: 0x9C27B0,
        },
    });
    label.position.set(15, 395);
    app.stage.addChild(label);

    app.ticker.add(() => {
    });

    Object.values(Regina.buttons).forEach((btn) => {
        const el = document.getElementById(btn.visualId);
        el?.addEventListener(Regina.events.click, () => {
            buttonClicked(btn.physicalUpper);
        });
    });

    window.addEventListener(Regina.events.keydown, (event) => {
        const key = event.key.toLowerCase();

        const match = Object.values(Regina.buttons).find(
            (b) => b.physicalLower === key
        );

        if (match) buttonClicked(match.physicalUpper);
    });

    function buttonClicked(btn: string) {
        switch (btn) {
            case Regina.buttons.H.physicalUpper:
                sprite_green.x -= Regina.buttonMovementOffset;
                break;
            case Regina.buttons.J.physicalUpper:
                sprite_green.y += Regina.buttonMovementOffset;
                break;
            case Regina.buttons.K.physicalUpper:
                sprite_green.y -= Regina.buttonMovementOffset;
                break;
            case Regina.buttons.L.physicalUpper:
                sprite_green.x += Regina.buttonMovementOffset;
                break;
        }

        const message = `[ ${btn} ${sprite_green.x} ${sprite_green.y} ] ${crypto.randomUUID()}`;
        logInfo(message);
    }

    function logInfo(message: string) {
        if (Regina.logLevel === FebruaryLogLevel.Info) {
            console.log(message);
        }
    }
})();