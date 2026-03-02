declare const PIXI: any;
import {HopenanButton, HopenanLogLevel, HopenanSprite, HopenanString} from './element.js';

export const Regina = {
    logLevel: HopenanLogLevel.Trace as HopenanLogLevel,
    buttons: {
        digits: {
            D1: new HopenanButton('1', '1'),
            D2: new HopenanButton('2', '2'),
            D3: new HopenanButton('3', '3'),
            D4: new HopenanButton('4', '4'),
            D5: new HopenanButton('5', '5'),
            D6: new HopenanButton('6', '6'),
            D7: new HopenanButton('7', '7'),
            D8: new HopenanButton('8', '8'),
            D9: new HopenanButton('9', '9'),
            D0: new HopenanButton('0', '0'),
        },
        movement: {
            H: new HopenanButton('h', 'H'),
            J: new HopenanButton('j', 'J'),
            K: new HopenanButton('k', 'K'),
            L: new HopenanButton('l', 'L'),
        },
    },
    spriteMovementOffset: 10,
    events: {
        click: 'click',
        keydown: 'keydown',
    },
    font: {
        name: 'HopenanTerminalRegular',
        url: 'https://igordurbek.b-cdn.net/IosevkaTerm-Regular.ttf',
        size: 72,
    },
    sprites: {
        green: new HopenanSprite('alice1a_green.png'),
        blue: new HopenanSprite('alice1a_blue.png'),
        red: new HopenanSprite('alice1a_red.png'),
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
        color: 0x9C27B0,
    },
    app: {
        div: 'hopenan',
        backgroundColorString: '#455A64'
    },
    grid: {
        stroke: {color: 0x212121, width: 1.2},
        fillColor: 0x757575,
    },

    getElementById: (id: HopenanString) => document.getElementById(id.str) as HTMLElement,
    logTrace: (message: HopenanString) => {
        if (Regina.logLevel === HopenanLogLevel.Trace || Regina.logLevel === HopenanLogLevel.Info) {
            console.log(message.str);
        }
    },
    logInfo: (message: HopenanString) => {
        if (Regina.logLevel === HopenanLogLevel.Info) {
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
    getButtonClickedMessage: (btn: HopenanString, sprite: HopenanSprite): HopenanString =>
        new HopenanString(`[ ${btn.str} ${sprite.x} ${sprite.y} ] ${crypto.randomUUID()}`),
} as const;

async function loadMyFont() {
    const font = new FontFace(
        Regina.font.name,
        `${Majic.fontSource(Regina.font.url)}`
    );

    const loaded = await font.load();
    (document.fonts as unknown as FontFaceSet).add(loaded);

    await document.fonts.load(Majic.documentFontsLoad(Regina.font.size, Regina.font.name));

    Regina.logTrace(new HopenanString(Majic.documentFontLoaded(Regina.font.name)));
}

(async () => {
    const div = Regina.getElementById(new HopenanString(Regina.app.div));
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
        text: `(${Regina.spriteMovementOffset})`,
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

    const buttons: readonly HopenanButton[] = Object.values(Regina.buttons).flatMap(
        (group) => Object.values(group)
    );
    for (const btn of buttons) {
        const el = document.getElementById(btn.visualId);
        el?.addEventListener(Regina.events.click, () => {
            buttonClicked(btn.physicalUpper);
        });
    }

    window.addEventListener(Regina.events.keydown, (event) => {
        const key = event.key.toLowerCase();

        const match = buttons.find(
            (b) => b.physicalLower === key
        );

        if (match) buttonClicked(match.physicalUpper);
    });

    function buttonClicked(btn: string) {
        switch (btn) {
            case Regina.buttons.digits.D1.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D1.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D2.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D2.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D3.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D3.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D4.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D4.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D5.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D5.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D6.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D6.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D7.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D7.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D8.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D8.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D9.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D9.physicalUpper} ]`);
                break;
            case Regina.buttons.digits.D0.physicalUpper:
                console.log(`[ ${Regina.buttons.digits.D0.physicalUpper} ]`);
                break;
            case Regina.buttons.movement.H.physicalUpper:
                const newX1 = Regina.sprites.green.x - Regina.spriteMovementOffset
                Regina.sprites.green.setPosition(newX1, Regina.sprites.green.y);
                break;
            case Regina.buttons.movement.J.physicalUpper:
                const newY2 = Regina.sprites.green.y + Regina.spriteMovementOffset;
                Regina.sprites.green.setPosition(Regina.sprites.green.x, newY2);
                break;
            case Regina.buttons.movement.K.physicalUpper:
                const newY3 = Regina.sprites.green.y - Regina.spriteMovementOffset;
                Regina.sprites.green.setPosition(Regina.sprites.green.x, newY3);
                break;
            case Regina.buttons.movement.L.physicalUpper:
                const newX4 = Regina.sprites.green.x + Regina.spriteMovementOffset;
                Regina.sprites.green.setPosition(newX4, Regina.sprites.green.y);
                break;
        }

        Regina.logInfo(Majic.getButtonClickedMessage(new HopenanString(btn), Regina.sprites.green));
    }
})();