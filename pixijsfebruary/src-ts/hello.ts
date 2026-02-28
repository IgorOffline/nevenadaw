declare const PIXI: any;

class FebruarySprite {
    texture: string;
    sprite: any;
    x: number;
    y: number;

    constructor(texture: string) {
        this.texture = texture;
        this.sprite = null;
        this.x = 0;
        this.y = 0;
    }

    setPosition(x: number, y: number) {
        this.x = x;
        this.y = y;
        this.sprite.position.set(x, y);
    }
}

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

class FebruaryString {
    str: string;

    constructor(str: string) {
        this.str = str;
    }
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
    sprites: {
        green: new FebruarySprite('alice1a_green.png'),
        blue: new FebruarySprite('alice1a_blue.png'),
        red: new FebruarySprite('alice1a_red.png'),
    },
    texture: {
        default: {
            topLeft: 50,
            bottomRight: 300,
            scale: 0.15,
        }
    },
    label: {
        positionX: 15,
        positionY: 395,
        text: '123456789',
        color: 0x9C27B0,
    },
    app: {
        div: 'pixijsfebruary',
        backgroundColorString: '#455A64'
    },
    grid: {
        stroke: {color: 0x212121, width: 1.2},
        fillColor: 0x757575,
    },

    getElementById: (id: FebruaryString) => document.getElementById(id.str) as HTMLElement,
    logTrace: (message: FebruaryString) => {
        if (Regina.logLevel === FebruaryLogLevel.Trace || Regina.logLevel === FebruaryLogLevel.Info) {
            console.log(message.str);
        }
    },
    logInfo: (message: FebruaryString) => {
        if (Regina.logLevel === FebruaryLogLevel.Info) {
            console.log(message.str);
        }
    },
} as const;

export const Majic = {
    grid: {
        size: 69,
        rect: {size: 10, offset: 30},
    },
    fontSource: (fontUrl: string) => `url(${fontUrl})`,
    documentFontsLoad: (fontSize: any, fontName: any) => `${fontSize}px "${fontName}`,
    documentFontLoaded: (fontName: any) => `Font loaded: ${fontName}`,
    getButtonClickedMessage: (btn: FebruaryString, sprite: FebruarySprite): FebruaryString =>
        new FebruaryString(`[ ${btn} ${sprite.x} ${sprite.y} ] ${crypto.randomUUID()}`),
} as const;

async function loadMyFont() {
    const font = new FontFace(
        Regina.font.name,
        `${Majic.fontSource(Regina.font.url)}`
    );

    const loaded = await font.load();
    (document.fonts as unknown as FontFaceSet).add(loaded);

    await document.fonts.load(Majic.documentFontsLoad(Regina.font.size, Regina.font.name));

    Regina.logTrace(new FebruaryString(Majic.documentFontLoaded(Regina.font.name)));
}

(async () => {
    const div = Regina.getElementById(new FebruaryString(Regina.app.div));
    if (!div) return;

    await loadMyFont();

    const app = new PIXI.Application();
    await app.init({
        background: Regina.app.backgroundColorString,
        resizeTo: div,
    });

    const g = new PIXI.Graphics();
    for (let ix = 0; ix < Majic.grid.size; ix++) {
        for (let jy = 0; jy < Majic.grid.size; jy++) {
            g.rect(-100 + ix * Majic.grid.rect.offset, -100 + jy * Majic.grid.rect.offset, Majic.grid.rect.size, Majic.grid.rect.size);
            g.fill(Regina.grid.fillColor);
            g.stroke({color: Regina.grid.stroke.color, width: Regina.grid.stroke.width});
        }
    }
    app.stage.addChild(g);

    div.appendChild(app.canvas);

    const texture_green = await PIXI.Assets.load(Regina.sprites.green.texture);
    const sprite_green = new PIXI.Sprite(texture_green);
    Regina.sprites.green.sprite = sprite_green;
    Regina.sprites.green.setPosition(Regina.texture.default.topLeft, Regina.texture.default.topLeft);
    sprite_green.scale.set(Regina.texture.default.scale);
    app.stage.addChild(sprite_green);

    const texture_blue = await PIXI.Assets.load(Regina.sprites.blue.texture);
    const sprite_blue = new PIXI.Sprite(texture_blue);
    Regina.sprites.blue.sprite = sprite_blue;
    Regina.sprites.blue.setPosition(Regina.texture.default.topLeft, Regina.texture.default.bottomRight);
    sprite_blue.scale.set(Regina.texture.default.scale);
    app.stage.addChild(sprite_blue);

    const texture_red = await PIXI.Assets.load(Regina.sprites.red.texture);
    const sprite_red = new PIXI.Sprite(texture_red);
    Regina.sprites.red.sprite = sprite_red;
    Regina.sprites.red.setPosition(Regina.texture.default.bottomRight, Regina.texture.default.bottomRight);
    sprite_red.scale.set(Regina.texture.default.scale);
    app.stage.addChild(sprite_red);

    const label = new PIXI.Text({
        text: Regina.label.text,
        style: {
            fontFamily: Regina.font.name,
            fontSize: Regina.font.size,
            fill: Regina.label.color,
        },
    });
    label.position.set(Regina.label.positionX, Regina.label.positionY);
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
                const newX1 = Regina.sprites.green.x - Regina.buttonMovementOffset
                Regina.sprites.green.setPosition(newX1, Regina.sprites.green.y);
                break;
            case Regina.buttons.J.physicalUpper:
                const newY2 = Regina.sprites.green.y + Regina.buttonMovementOffset;
                Regina.sprites.green.setPosition(Regina.sprites.green.x, newY2);
                break;
            case Regina.buttons.K.physicalUpper:
                const newY3 = Regina.sprites.green.y - Regina.buttonMovementOffset;
                Regina.sprites.green.setPosition(Regina.sprites.green.x, newY3);
                break;
            case Regina.buttons.L.physicalUpper:
                const newX4 = Regina.sprites.green.x + Regina.buttonMovementOffset;
                Regina.sprites.green.setPosition(newX4, Regina.sprites.green.y);
                break;
        }

        Regina.logInfo(Majic.getButtonClickedMessage(new FebruaryString(btn), Regina.sprites.green));
    }
})();