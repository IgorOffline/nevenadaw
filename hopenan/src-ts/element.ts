export class HopenanButton {
    physicalLower: string;
    physicalUpper: string;
    visualId: string;

    constructor(physicalLower: string, physicalUpper: string) {
        this.physicalLower = physicalLower;
        this.physicalUpper = physicalUpper;
        this.visualId = `btn-${physicalLower}`;
    }
}

export enum HopenanLogLevel {
    Trace = 0,
    Info = 1,
}

export class HopenanSprite {
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

export class HopenanString {
    str: string;

    constructor(str: string) {
        this.str = str;
    }
}
