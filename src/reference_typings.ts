/** Namespace describing each ShaderLanguage type option in a ShaderYY file */
export namespace ShaderLanguageType {
  export const GLSLES: 0 = 0;
  export const GLSL: 2 = 2;
  export const HLSL: 4 = 4;
}
export type ShaderLanguageType = 0|2|4;

/** Unknown purpose, as opposed to true boolean. Used in Rooms */
export namespace PseudoBoolean {
  export const False: 0 = 0;
  export const True: 1 = 1;
}
export type PseudoBoolean = 0|1;

/** Sound 'kind' used in a SoundYY file */
export namespace SoundKind {
  export const UncompressedNotStreamed: 0 = 0;
  export const CompressedNotStreamed: 1 = 1;
  export const UncompressOnLoad: 2 = 2;
  export const CompressedStreamed: 3 = 3;
}
export type SoundKind = 0|1|2|3;

/** Sound 'type' used in a SoundYY file */
export namespace SoundType {
  export const Mono: 0 = 0;
  export const Stereo: 1 = 1;
  export const ThreeDee: 2 = 2;
}
export type SoundType = 0|1|2;

/** Silly enum used for Argument type in an Extension */
export namespace ArgumentType {
  export const StringType: 1 = 1;
  export const DoubleType: 2 = 2;
}
export type ExtensionFunctionType = 1|2;

/** Path 'kind' used in a PathYY file */
export namespace PathKind {
  export const Straight: 0 = 0;
  export const Smooth: 1 = 1;
}
export type PathKind = 0|1;

export namespace SpriteBBoxMode {
  export const Automatic: 0 = 0;
  export const FullImage: 1 = 1;
  export const Manual: 2 = 2;
}
export type SpriteBBoxMode = 0|1|2;

export namespace SpriteCollisionKind {
  export const Rectangle: 1 = 1;
  export const RotatedRectangle: 5 = 5;
  export const Ellipse: 2 = 2;
  export const Diamond: 3 = 3;
  export const Precise: 0 = 0;
  export const PrecisePerFrame: 0 = 0;
}
export type SpriteCollisionKind = 0|1|2|3|5;

export namespace SpritePlaybackSpeed {
  export const FramesPerSecond: 0 = 0;
  export const FramesPerGameFrame: 1 = 1;
}
export type SpritePlaybackSpeed = 0|1;

/**
 * Event 'type' used in  Events inside ObjectYY files. Within each 'type'
 *  there is also a `number` which describes a "sub-type" of the event.
 *  Use `EventNumber` for that subtype.
 */
export namespace EventType {
  export const Create: 0 = 0;
  export const Destroy: 1 = 1;
  export const Alarm: 2 = 2;
  export const Step: 3 = 3;
  export const Collision: 4 = 4;
  export const Keyboard: 5 = 5;
  export const Mouse: 6 = 6;
  export const Other: 7 = 7;
  export const Draw: 8 = 8;
  export const KeyPress: 9 = 9;
  export const KeyRelease: 10 = 10;
  export const Trigger: 11 = 11;
  export const CleanUp: 12 = 12;
  export const Gesture: 13 = 13;
}
export type EventType =|0|1|2|3|4|5|6|7|8|9|10|11|12|13;

/**
 * Event `number` used in Events in ObjectYY files. Each
 * `EventType` only has some of these members.
 */
export namespace EventNumber {
  export const Create: 0 = 0;
  export const DrawNormal: 0 = 0;
  export const DrawBegin: 72 = 72;
  export const DrawEnd: 73 = 73;
  export const DrawPre: 76 = 76;
  export const DrawPost: 77 = 77;
  export const LeftButton: 0 = 0;
  export const RightButton: 1 = 1;
  export const MiddleButton: 2 = 2;
  export const NoButton: 3 = 3;
  export const LeftPress: 4 = 4;
  export const RightPress: 5 = 5;
  export const MiddlePress: 6 = 6;
  export const LeftRelease: 7 = 7;
  export const RightRelease: 8 = 8;
  export const MiddleRelease: 9 = 9;
  export const MouseEnter: 10 = 10;
  export const MouseLeave: 11 = 11;
  export const MouseWheelUp: 60 = 60;
  export const MouseWheelDown: 61 = 61;
  export const GlobalLeftButton: 50 = 50;
  export const GlobalRightButton: 51 = 51;
  export const GlobalMiddleButton: 52 = 52;
  export const GlobalLeftPress: 53 = 53;
  export const GlobalRightPress: 54 = 54;
  export const GlobalMiddlePress: 55 = 55;
  export const GlobalLeftRelease: 56 = 56;
  export const GlobalRightRelease: 57 = 57;
  export const GlobalMiddleRelease: 58 = 58;
  export const Joystick1Left: 16 = 16;
  export const Joystick1Right: 17 = 17;
  export const Joystick1Up: 18 = 18;
  export const Joystick1Down: 19 = 19;
  export const Joystick1Button1: 21 = 21;
  export const Joystick1Button2: 22 = 22;
  export const Joystick1Button3: 23 = 23;
  export const Joystick1Button4: 24 = 24;
  export const Joystick1Button5: 25 = 25;
  export const Joystick1Button6: 26 = 26;
  export const Joystick1Button7: 27 = 27;
  export const Joystick1Button8: 28 = 28;
  export const Joystick2Left: 31 = 31;
  export const Joystick2Right: 32 = 32;
  export const Joystick2Up: 33 = 33;
  export const Joystick2Down: 34 = 34;
  export const Joystick2Button1: 36 = 36;
  export const Joystick2Button2: 37 = 37;
  export const Joystick2Button3: 38 = 38;
  export const Joystick2Button4: 39 = 39;
  export const Joystick2Button5: 40 = 40;
  export const Joystick2Button6: 41 = 41;
  export const Joystick2Button7: 42 = 42;
  export const Joystick2Button8: 43 = 43;
  export const Outside: 0 = 0;
  export const Boundary: 1 = 1;
  export const GameStart: 2 = 2;
  export const GameEnd: 3 = 3;
  export const RoomStart: 4 = 4;
  export const RoomEnd: 5 = 5;
  export const NoMoreLives: 6 = 6;
  export const AnimationEnd: 7 = 7;
  export const EndOfPath: 8 = 8;
  export const NoMoreHealth: 9 = 9;
  export const CloseButton: 30 = 30;
  export const User0: 10 = 10;
  export const User1: 11 = 11;
  export const User2: 12 = 12;
  export const User3: 13 = 13;
  export const User4: 14 = 14;
  export const User5: 15 = 15;
  export const User6: 16 = 16;
  export const User7: 17 = 17;
  export const User8: 18 = 18;
  export const User9: 19 = 19;
  export const User10: 20 = 20;
  export const User11: 21 = 21;
  export const User12: 22 = 22;
  export const User13: 23 = 23;
  export const User14: 24 = 24;
  export const User15: 25 = 25;
  export const StepNormal: 0 = 0;
  export const StepBegin: 1 = 1;
  export const StepEnd: 2 = 2;
  export const Gui: 64 = 64;
  export const GuiBegin: 74 = 74;
  export const GuiEnd: 7 = 7;
  export const WindowResize: 65 = 65;
  export const AsyncAudioPlayBack: 74 = 74;
  export const AsyncAudioRecording: 73 = 73;
  export const AsyncCloud: 67 = 67;
  export const AsyncDialog: 63 = 63;
  export const AsyncHTTP: 62 = 62;
  export const AsyncInAppPurchase: 66 = 66;
  export const AsyncImageLoaded: 60 = 60;
  export const AsyncNetworking: 68 = 68;
  export const AsyncPushNotification: 71 = 71;
  export const AsyncSaveLoad: 72 = 72;
  export const AsyncSocial = 70;
  export const AsyncSteam: 69 = 69;
  export const AsyncSystem: 75 = 75;
}
export type EventNumber =|72|73|76|77|0|1|2|3|4|5|6|7|8|9|10|11|60|61|50|51|52|
    53|54|55|56|57|58|16|17|18|19|21|22|23|24|25|26|27|28|31|32|33|34|36|37|38|
    39|40|41|42|43|64|74|7|65|75|74|73|72|71|70|69|68|67|66|63|62|60;

/** Resource file namespace */
export namespace Resource {
  export declare type ModelNames = | 'GMObject' | 'GMIncludedFile' |
      'GMExtension' | 'GMExtensionFile' | 'GMExtensionFunction' |
      'GMExtensionConstant' | 'GMFont' | 'GMNote' | 'GMOption' | 'GMPath' |
      'GMRoom' | 'GMScript' | 'GMShader' | 'GMSound' | 'GMSprite' |
      'GMTileSet' | 'GMFolder' | 'GMTimeline';

  export interface BaseResource {
    /** Event GUID */
    id: string;

    /** Internal resource type descriptor (GMEvent) */
    modelName: ModelNames;

    /** Version string, unknown use */
    mvc: string;

    /** Resource name */
    name: string;
  }

  export interface ObjectEvent {
    /** Event GUID */
    id: string;

    /** Internal resource type descriptor (GMEvent) */
    modelName: string;

    /** Version string, unknown use */
    mvc: string;

    /** Indicates if this event is drag and drop */
    IsDnD: boolean;

    /**
     * Presumably, this holds the GUID of the other object if this were a
     * collision event
     */
    collisionObjectId: string;

    /** Internal sub-event number */
    enumb: EventNumber;

    /** Internal Event number */
    eventtype: EventType;

    /** GUID of the object that owns this event (Can belong to parent object) */
    m_owner: string;
  }

  export interface Object extends BaseResource {
    /** Internal resource type descriptor (GMObject) */
    modelName: 'GMObject';

    /** List of object events */
    eventList: Array<ObjectEvent>;

    /** GUID of sprite mask */
    maskSpriteId: string;

    /** Properties overriden. */
    overriddenProperties: Array<object>|null;

    /** Properties are variables set using the "variables" box in the IDE. */
    properties: Array<object>|null;

    /** GUID of parent object */
    parentObjectId: string;

    /** Indicates if this object is persistent */
    persistent: boolean;

    /** Indicates if this object uses physics */
    physicsObject: boolean;

    physicsAngularDamping: number;
    physicsDensity: number;
    physicsFriction: number;
    physicsGroup: number;
    physicsKinematic: boolean;
    physicsLinearDamping: number;
    physicsRestitution: number;
    physicsSensor: boolean;
    physicsShape: number;
    physicsShapePoints: Array<any>|null;
    physicsStartAwake: boolean;

    /** Indicates if this object is solid */
    solid: boolean;

    /** GUID of this object's sprite */
    spriteId: string;

    /** Indicates if this object is visible */
    visible: boolean;
  }

  export interface Options extends BaseResource {
    /** Internal resource type descriptor */
    modelName: 'GMOption';
  }

  export interface Room extends BaseResource {
    /**
     * The name of the Room Creation code FP, relative to the Room folder
     * itself.
     */
    creationCodeFile: string;

    /** Inherit Code from a Parent Room */
    inheritCode: boolean;

    /** Inherit Creation Order from a Parent Room */
    inheritCreationOrder: boolean;

    /** Inherit Layers from a Parent Room */
    inheritLayers: boolean;

    /** instanceCreationOrderIDs */
    instanceCreationOrderIDs: Array<string>;
    isDnD: boolean;

    /** All your layers are placed here. */
    layers: Array<Layer>;

    /** Internal resource type descriptor */
    modelName: 'GMRoom';

    /** Parent Room ID */
    parentID: string;

    /** Physics setting of the room. */
    physicsSettings: PhysicsSettings;
  }

  export interface Layer extends BaseResource {
    __type: string;
    depth: number;
    grid_x: number;
    grid_y: number;
    hierarchyFrozen: boolean;
    hierarchyVisible: boolean;
    inheritLayerDepth: boolean;
    inheritLayerSettings: boolean;
    inheritSubLayers: boolean;
    inheritVisibility: boolean;
    instances: Array<Instance>;
    layers: Array<Layer>;
    m_parentID: string;
    m_serialiseFrozen: boolean;
    userdefined_depth: boolean;
    visible: boolean;
  }

  export interface Instance extends BaseResource {
    colour: Colour;
    creationCodeFile: string;
    creationCodeType: string;
    ignore: boolean;
    inheritCode: boolean;
    inheritItemSettings: boolean;
    IsDnD: boolean;
    m_originalParentID: string;
    m_serialiseFrozen: boolean;
    name_with_no_file_rename: string;
    objId: string;
    properties: null;
    rotation: number;
    scaleX: number;
    scaleY: number;
    mvc: string;
    x: number;
    y: number;
  }

  export interface Colour {
    Value: number;
  }

  export interface PhysicsSettings {
    id: string;
    inheritPhysicsSettings: boolean;
    modelName: string;
    PhysicsWorld: boolean;
    PhysicsWorldGravityX: number;
    PhysicsWorldGravityY: number;
    PhysicsWorldPixToMeters: number;
    mvc: string;
  }
  /**
   * We type it as "roomSetting" to reflect the .YY's typing of it.
   */
  export interface roomSettings {
    id: string;
    Height: number;
    inheritRoomSettings: boolean;
    modelName: string;
    persistent: boolean;
    mvc: string;
    Width: number;
  }

  export interface View {
    id: string;
    hborder: number;
    hport: number;
    hspeed: number;
    hview: number;
    inherit: boolean;
    modelName: string;
    objId: string;
    mvc: string;
    vborder: number;
    visible: boolean;
    vspeed: number;
    wport: number;
    wview: number;
    xport: number;
    xview: number;
    yport: number;
    yview: number;
  }

  export interface viewSettings {
    id: string;
    clearDisplayBuffer: boolean;
    clearViewBackground: boolean;
    enableViews: boolean;
    inheritViewSettings: boolean;
    modelName: string;
    mvc: string;
  }

  /**
   * Bare bones interface for now. A Sprite has a lot more
   * to it than just the BaseResource. Will be added soon.
   */
  export interface Sprite extends BaseResource {
    /** Internal resource type descriptor */
    modelName: 'GMSprite';

    For3D: boolean;
    HTile: boolean;
    VTile: boolean;
    bbox_bottom: number;
    bbox_left: number;
    bbox_right: number;
    bbox_top: number;
    bboxmode: SpriteBBoxMode;
    colkind: SpriteCollisionKind;
    coltolerance: number;
    edgeFiltering: boolean;
    frames: Frame[];
    layers: ImageLayer[];
    gridX: number;
    gridY: number;
    height: number;
    origin: number;
    originLocked: boolean;
    playbackSpeed: number;
    playbackSpeedtype: SpritePlaybackSpeed;
    premultiplyAlpha: boolean;
    sepmasks: boolean;
    swatchColours: null;
    swfPrecision: number;
    textureGroupId: string;
    type: 0;
    width: number;
    xorig: number;
    yorig: number;
  }

  export interface Frame {
    id: string;
    modelName: 'GMSpriteFrame';
    mvc: '1.0';
    SpriteID: string;
    compositeImage: SpriteImage;
    images: SpriteImage[];
  }

  export interface SpriteImage {
    id: string;
    modelName: 'GMSpriteImage';
    mvc: '1.0';
    FrameId: string;
    LayerId: string;
  }

  export interface ImageLayer {
    id: string;
    modelName: 'GMImageLayer';
    mvc: string;
    SpriteId: string;
    blendMode: number;
    isLocked: boolean;
    name: string;
    opacity: number;
    visible: boolean;
  }

  export interface Sound extends BaseResource {
    modelName: 'GMSound';

    /**
     * The GUID of the audio group. Unknown where audio group data itself is
     * stored.
     */
    audioGroupGuid: string;

    /** Quality of the sound, set in the IDE at 8bit or 16bit. */
    bitDepth: number;

    /** The Bit Rate in kbps. */
    bitRate: number;

    /** The "Attribute" of the sound. */
    kind: SoundKind;

    preLoad: boolean;
    sampleRate: number;
    type: SoundType;
    volume: number;
  }

  export interface Path extends BaseResource {
    modelName: 'GMPath';

    /** Path closed or open */
    closed: boolean;

    hsnap: number;

    /** Straight or smooth path. */
    kind: PathKind;
    points: Array<PathPoint>;
    precision: number;
    vsnap: 0;
  }

  export interface PathPoint {
    /** Resource GUID */
    id: string;

    /** Internal resource type descriptor */
    modelName: string;

    /** Version string, unknown use */
    mvc: string;

    x: number;
    y: number;
    speed: number;
  }

  export interface GMFolder extends BaseResource {
    /** Internal resource type descriptor */
    modelName: 'GMFolder';

    /** An array of the views/resource GUIDs which this folder contains. */
    children: Array<string>;

    /** The FilterType of the View */
    filterType: string;

    /** The folder name itself */
    folderName: string;

    /** Indicates if the view is the Default Node. */
    isDefaultView: boolean;

    /** A code, likely used for adding localizations. */
    localisedFolderName: localisedNames;
  }
  export type localisedNames =
      |''|'ResourceTree_Fonts'|'ResourceTree_Scripts'|'ResourceTree_Notes'|
      'ResourceTree_Sounds'|'ResourceTree_Tilesets'|'ResourceTree_Configs'|
      'ResourceTree_Paths'|'ResourceTree_Sprites'|'ResourceTree_IncludedFiles'|
      'ResourceTree_Extensions'|'ResourceTree_Objects'|'ResourceTree_Options'|
      'ResourceTree_Timelines'|'ResourceTree_Shaders'|'ResourceTree_Rooms';

  export interface Tileset extends BaseResource {
    /** Internal resource type descriptor */
    modelName: 'GMTileSet';
  }

  export interface Script extends BaseResource {
    modelName: 'GMScript';
    IsCompatibility: boolean;
    IsDnD: boolean;
  }

  export interface Font extends BaseResource {
    /** Internal resource type descriptor */
    modelName: 'GMFont';

    /**
     * Checks if AntiAliasing is enabled. Not a real boolean, but always 0
     * or 1.
     */
    AntiAlias: PseudoBoolean;

    /** Unknown use. Likely related to TTFs */
    TTFName: string;

    bold: boolean;
    charset: number;
    first: number;
    fontName: string;
    glyphs: Array<Glyph>;
    /** Unknown usage. */
    image: null;
    includeTTF: boolean;
    italic: boolean;
    /** Unknown usage. */
    kerningPairs: Array<any>;
    last: number;
    ranges: Array<{x: number; y: number;}>;
    sampleText: string;
    size: number;
    styleName: string;
    textureGroupId: string;
  }

  export interface Glyph {
    Key: number;
    Value: {
      id: string; modelName: 'GMGlyph'; mvc: string; character: number;
      h: number;
      offset: number;
      shift: number;
      w: number;
      x: number;
      y: number;
    };
  }

  export interface Timeline extends BaseResource {
    /** Internal resource type descriptor */
    modelName: 'GMTimeline';

    /** Array of "moments" in the timeline */
    momentList: Array<Moment>;
  }

  export interface Moment {
    modelName: 'GMMoment';

    /**
     * Describes the .gml file for each moment. Coded as a Create event.
     * Yes, this is how it is spelled.
     */
    evnt: ObjectEvent;
    moment: number;
  }

  export interface Note extends BaseResource {
    modelName: 'GMNote';
  }

  export interface Extension extends BaseResource {
    modelName: 'GMExtension';
    /**
     * This is where the extension creator specifies what extra resources are
     * there.
     */
    IncludedResources: any[];

    files: Array<GMExtensionFile>;
  }

  export interface GMExtensionFile extends BaseResource {
    modelName: 'GMExtensionFile';

    /** Array of ProxyFiles. Unknown usage. */
    ProxyFiles: Array<any>;

    /** These are the constants, or macros, which the extension provides. */
    constants: Array<GMExtensionConstant>;

    /** These are the functions which the extension provides. */
    functions: Array<GMExtensionFunction>;

    /** Specifies to which targets to compile extension. Bit. */
    copyToTargets: number;

    /** This is the name of the file which will be in the same folder. */
    filename: string;

    /** The initial function called. */
    init: string;

    /** The final function called by the extension. */
    final: string;

    /** This is the type of the Extension. It is unclear what the types are. */
    kind: number;

    /**
     * Order of the functions. The strings here refer to the UUIDs of the
     * functions, which is their ID.
     */
    order: Array<string>;

    /** The original name of the function. Unknown usage. */
    origname: string;

    /** Whether it is compressed. Unknown usage. */
    uncompress: boolean;
  }

  export interface GMExtensionConstant extends BaseResource {
    modelName: 'GMExtensionConstant';

    /** The name of the Macro */
    constantName: string;

    /** If the macro should be hidden from the user */
    hidden: boolean;

    /**
     * This is a GML snippet which the Macro will be replaced with. It is
     * identical to #macro someMacro THIS_IS_THE_SNIPPET.
     */
    value: string;
  }

  export interface GMExtensionFunction extends BaseResource {
    modelName: 'GMExtensionFunction';

    /**
     * The number of arguments which the extension has. Note: -1 indicates that
     * a variable number of arguments are accepted.
     */
    argCount: number;

    /** Array of argument type. See @type ArgumentType for these: */
    args: ExtensionFunctionType[];

    /** The external name of the function. */
    externalName: string;

    /**
     * Help is the popup which will come up in GMS2's autocomplete. It is
     * essentially a signature line.
     */
    help: string;

    /** Controls if the function is visible to the user or not. */
    hidden: boolean;

    /**
     * This is the type of the Extension, inherited. It is unclear what the
     * types are.
     */
    kind: number;

    /** This is the name as the user will see it of the function */
    name: string;

    /** This is the ArgumentType return. */
    returnType: ExtensionFunctionType;
  }

  export interface Shader extends BaseResource {
    modelName: 'GMShader';

    /** Shader language used. */
    type: ShaderLanguageType;
  }

  /**
   * Generic type which can refer to any resource type. Use when you know
   * that the BaseResource has what you need, which all of these contain.
   */
  export type GMResource =|GMFolder|Sprite|Tileset|Sound|Path|Script|Shader|
      Font|Timeline|Object|Room|Note|Extension;
}

/** Parent project entry of a YYP */
export interface ParentProject {
  /** GUID of the parent project */
  id: string;

  /** Describes object entry type. */
  modelName: 'GMProjectParent';

  /** A version number string, unknown use */
  mvc: string;

  /** Contains parent project resources */
  alteredResources: Array<YYPResource>;

  /** Unkown property, usually an empty array */
  hiddenResources: Array<YYPResource>;

  /**
   * Contains parent project path presumably, always contains the following
   * string: "${base_project}"
   */
  projectPath: string;
}

/** Represents a resource entry in a YYP */
export interface YYPResource {
  /**
   * This resource entry GUID (not the GUID of the resource itself). Appears to
   * serve no purpose.
   */
  Key: string;

  /** Contains resource information */
  Value: {
    /** GUID of the resource */
    id: string;

    /**
       Describes object entry type, which is always "GMResourceInfo" for
       YYPResources
     */
    modelName: 'GMResourceInfo';

    /** A version number string, unknown use */
    mvc: string;

    /** Unknown property, seems to always be an empty array */
    configDeltaFiles: Array<any>;

    /** Unknown property, seems to always be an empty array */
    configDeltas: Array<any>;

    /** Unknown property, seems to always have only one entry: "default" */
    resourceCreationConfigs: Array<string>;

    /**
       Contains the relative backslash-escaped path to the resource's .yy file
     */
    resourcePath: string;

    /** Describes the resource type */
    resourceType: Resource.ModelNames;
  };
}

/** GMS2 project file typings */
export interface YYP {
  /** Contains project GUID */
  id: string;

  /** Usually contains resource type, in this case GMProject */
  modelName: 'GMProject';

  /** A version number string, unknown use */
  mvc: string;

  /** Denotes whether this project uses drag and drop or not */
  IsDnDProject: boolean;

  /** Unknown property, seems to always be an empty array */
  configs: Array<any>;

  /**
   * Allows for experimental JS editing. Unfinished or legacy feature. It's a
   * secret.
   */
  option_ecma: boolean;

  /** Parent project, apparently non-public feature */
  parentProject: ParentProject;

  /** Contains all project resources (unordered) */
  resources: Array<YYPResource>;

  /** An array of script GUID's, seemingly optional */
  script_order?: Array<string>;

  /** Unknown property, usually an empty string */
  tutorial?: string;
}