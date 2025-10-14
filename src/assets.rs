use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileAssets {
    #[allow(dead_code)]
    FontsEditundo,
    #[allow(dead_code)]
    ImagesGamePrimitivesRect,
    #[allow(dead_code)]
    ImagesGameGameBackground,
    #[allow(dead_code)]
    ImagesGameMenuBackground,
    #[allow(dead_code)]
    ImagesGameTilemapPacked,
    #[allow(dead_code)]
    ImagesGameUnitsTank,
    #[allow(dead_code)]
    ImagesGameUnitsAntiair,
    #[allow(dead_code)]
    ImagesGameUnitsApc,
    #[allow(dead_code)]
    ImagesGameUnitsArtillery,
    #[allow(dead_code)]
    ImagesGameUnitsBattlecruiser,
    #[allow(dead_code)]
    ImagesGameUnitsBcopter,
    #[allow(dead_code)]
    ImagesGameUnitsBomber,
    #[allow(dead_code)]
    ImagesGameUnitsCarrier,
    #[allow(dead_code)]
    ImagesGameUnitsCruiser,
    #[allow(dead_code)]
    ImagesGameUnitsFighter,
    #[allow(dead_code)]
    ImagesGameUnitsInfantry,
    #[allow(dead_code)]
    ImagesGameUnitsLander,
    #[allow(dead_code)]
    ImagesGameUnitsMaintank,
    #[allow(dead_code)]
    ImagesGameUnitsMech,
    #[allow(dead_code)]
    ImagesGameUnitsMissile,
    #[allow(dead_code)]
    ImagesGameUnitsRecon,
    #[allow(dead_code)]
    ImagesGameUnitsRocketlauncher,
    #[allow(dead_code)]
    ImagesGameUnitsSub,
    #[allow(dead_code)]
    ImagesGameUnitsSupplyship,
    #[allow(dead_code)]
    ImagesGameUnitsTcopter,
    #[allow(dead_code)]
    ImagesGameUnitsWartank,
    #[allow(dead_code)]
    ImagesGameTilemap2,
    #[allow(dead_code)]
    ImagesGameEffectsExplosion,
    #[allow(dead_code)]
    ImagesGameEffectsBomb,
    #[allow(dead_code)]
    ImagesGameEffectsGunattack,
    #[allow(dead_code)]
    ImagesGameEffectsSmallExplosion,
    #[allow(dead_code)]
    ImagesGameEffectsTorpedo,
    #[allow(dead_code)]
    ImagesGameEffectsVulcanCannon,
    #[allow(dead_code)]
    ImagesGameCursorHud36X36,
    #[allow(dead_code)]
    ImagesGameCursorHud38X38,
    #[allow(dead_code)]
    ImagesGameCursorAttack40X40,
    #[allow(dead_code)]
    ImagesGameCursorAttack42X42,
    #[allow(dead_code)]
    ImagesGameMovementHud,
    #[allow(dead_code)]
    ImagesGameTerrain,
    #[allow(dead_code)]
    ImagesIconsNodesCarOutline,
    #[allow(dead_code)]
    ImagesIconsNodesFlagOutline,
    #[allow(dead_code)]
    ImagesIconsNodesLayersOutline,
    #[allow(dead_code)]
    ImagesIconsNodesPaperPlaneOutline,
    #[allow(dead_code)]
    ImagesIconsNodesPersonOutline,
    #[allow(dead_code)]
    ImagesIconsNodesSettings2Outline,
    #[allow(dead_code)]
    ImagesIconsNodesDownloadOutline,
    #[allow(dead_code)]
    ImagesIconsUiIconsDefault,
    #[allow(dead_code)]
    ImagesIconsUiSheetBlack1X,
    #[allow(dead_code)]
    ImagesIconsUiSheetBlack2X,
    #[allow(dead_code)]
    ImagesIconsUiSheetWhite1X,
    #[allow(dead_code)]
    ImagesIconsUiSheetWhite2X,
    #[allow(dead_code)]
    ImagesThemeBlueSheet,
    #[allow(dead_code)]
    ImagesThemeCursor,
    #[allow(dead_code)]
    ImagesThemeGreenSheet,
    #[allow(dead_code)]
    ImagesThemeGreySheet,
    #[allow(dead_code)]
    ImagesThemeRedSheet,
    #[allow(dead_code)]
    ImagesThemeYellowSheet,
    #[allow(dead_code)]
    MusicsCyberpunkMoonlightSonataV2,
    #[allow(dead_code)]
    MusicsAwesomeness,
    #[allow(dead_code)]
    MusicsImmaculateCommandGeorgesDeMorgansThemeCodenameRecon,
    #[allow(dead_code)]
    MusicsTempestassAceInTheHoleMinatusLeonusMacrosThemeUnfinishedV08,
    #[allow(dead_code)]
    SoundsChipsHandle6,
    #[allow(dead_code)]
    SoundsEngineCircular000,
    #[allow(dead_code)]
    SoundsExplosionCrunch000,
    #[allow(dead_code)]
    SoundsExplosionCrunch004,
    #[allow(dead_code)]
    SoundsFootstepCarpet001,
    #[allow(dead_code)]
    SoundsHold,
    #[allow(dead_code)]
    SoundsImpactMining004,
    #[allow(dead_code)]
    SoundsImpactPlateMedium004,
    #[allow(dead_code)]
    SoundsLaserLarge004,
    #[allow(dead_code)]
    SoundsLaserRetro003,
    #[allow(dead_code)]
    SoundsLaserSmall001,
    #[allow(dead_code)]
    SoundsSpaceEngine001,
    #[allow(dead_code)]
    SoundsUiBack001,
    #[allow(dead_code)]
    SoundsUiBack002,
    #[allow(dead_code)]
    SoundsUiBack003,
    #[allow(dead_code)]
    SoundsUiBack004,
    #[allow(dead_code)]
    SoundsUiBong001,
    #[allow(dead_code)]
    SoundsUiClick001,
    #[allow(dead_code)]
    SoundsUiClick002,
    #[allow(dead_code)]
    SoundsUiClick003,
    #[allow(dead_code)]
    SoundsUiClick004,
    #[allow(dead_code)]
    SoundsUiClick005,
    #[allow(dead_code)]
    SoundsUiClose001,
    #[allow(dead_code)]
    SoundsUiClose002,
    #[allow(dead_code)]
    SoundsUiClose003,
    #[allow(dead_code)]
    SoundsUiClose004,
    #[allow(dead_code)]
    SoundsUiConfirmation001,
    #[allow(dead_code)]
    SoundsUiConfirmation002,
    #[allow(dead_code)]
    SoundsUiConfirmation003,
    #[allow(dead_code)]
    SoundsUiConfirmation004,
    #[allow(dead_code)]
    SoundsUiDoorOpen001,
    #[allow(dead_code)]
    SoundsUiDoorOpen002,
    #[allow(dead_code)]
    SoundsUiDrop001,
    #[allow(dead_code)]
    SoundsUiDrop002,
    #[allow(dead_code)]
    SoundsUiDrop003,
    #[allow(dead_code)]
    SoundsUiDrop004,
    #[allow(dead_code)]
    SoundsUiError001,
    #[allow(dead_code)]
    SoundsUiError002,
    #[allow(dead_code)]
    SoundsUiError003,
    #[allow(dead_code)]
    SoundsUiError004,
    #[allow(dead_code)]
    SoundsUiError005,
    #[allow(dead_code)]
    SoundsUiError006,
    #[allow(dead_code)]
    SoundsUiError007,
    #[allow(dead_code)]
    SoundsUiError008,
    #[allow(dead_code)]
    SoundsUiGlass001,
    #[allow(dead_code)]
    SoundsUiGlass002,
    #[allow(dead_code)]
    SoundsUiGlass003,
    #[allow(dead_code)]
    SoundsUiGlass004,
    #[allow(dead_code)]
    SoundsUiGlass005,
    #[allow(dead_code)]
    SoundsUiGlass006,
    #[allow(dead_code)]
    SoundsUiGlitch001,
    #[allow(dead_code)]
    SoundsUiGlitch002,
    #[allow(dead_code)]
    SoundsUiGlitch003,
    #[allow(dead_code)]
    SoundsUiGlitch004,
    #[allow(dead_code)]
    SoundsUiMaximize001,
    #[allow(dead_code)]
    SoundsUiMaximize002,
    #[allow(dead_code)]
    SoundsUiMaximize003,
    #[allow(dead_code)]
    SoundsUiMaximize004,
    #[allow(dead_code)]
    SoundsUiMaximize005,
    #[allow(dead_code)]
    SoundsUiMaximize006,
    #[allow(dead_code)]
    SoundsUiMaximize007,
    #[allow(dead_code)]
    SoundsUiMaximize008,
    #[allow(dead_code)]
    SoundsUiMaximize009,
    #[allow(dead_code)]
    SoundsUiMinimize001,
    #[allow(dead_code)]
    SoundsUiMinimize002,
    #[allow(dead_code)]
    SoundsUiMinimize003,
    #[allow(dead_code)]
    SoundsUiMinimize004,
    #[allow(dead_code)]
    SoundsUiMinimize005,
    #[allow(dead_code)]
    SoundsUiMinimize006,
    #[allow(dead_code)]
    SoundsUiMinimize007,
    #[allow(dead_code)]
    SoundsUiMinimize008,
    #[allow(dead_code)]
    SoundsUiMinimize009,
    #[allow(dead_code)]
    SoundsUiOpen001,
    #[allow(dead_code)]
    SoundsUiOpen002,
    #[allow(dead_code)]
    SoundsUiOpen003,
    #[allow(dead_code)]
    SoundsUiOpen004,
    #[allow(dead_code)]
    SoundsUiPluck001,
    #[allow(dead_code)]
    SoundsUiPluck002,
    #[allow(dead_code)]
    SoundsUiQuestion001,
    #[allow(dead_code)]
    SoundsUiQuestion002,
    #[allow(dead_code)]
    SoundsUiQuestion003,
    #[allow(dead_code)]
    SoundsUiQuestion004,
    #[allow(dead_code)]
    SoundsUiScratch001,
    #[allow(dead_code)]
    SoundsUiScratch002,
    #[allow(dead_code)]
    SoundsUiScratch003,
    #[allow(dead_code)]
    SoundsUiScratch004,
    #[allow(dead_code)]
    SoundsUiScratch005,
    #[allow(dead_code)]
    SoundsUiScroll001,
    #[allow(dead_code)]
    SoundsUiScroll002,
    #[allow(dead_code)]
    SoundsUiScroll003,
    #[allow(dead_code)]
    SoundsUiScroll004,
    #[allow(dead_code)]
    SoundsUiScroll005,
    #[allow(dead_code)]
    SoundsUiSelect001,
    #[allow(dead_code)]
    SoundsUiSelect002,
    #[allow(dead_code)]
    SoundsUiSelect003,
    #[allow(dead_code)]
    SoundsUiSelect004,
    #[allow(dead_code)]
    SoundsUiSelect005,
    #[allow(dead_code)]
    SoundsUiSelect006,
    #[allow(dead_code)]
    SoundsUiSelect007,
    #[allow(dead_code)]
    SoundsUiSelect008,
    #[allow(dead_code)]
    SoundsUiSwitch001,
    #[allow(dead_code)]
    SoundsUiSwitch002,
    #[allow(dead_code)]
    SoundsUiSwitch003,
    #[allow(dead_code)]
    SoundsUiSwitch004,
    #[allow(dead_code)]
    SoundsUiSwitch005,
    #[allow(dead_code)]
    SoundsUiSwitch006,
    #[allow(dead_code)]
    SoundsUiSwitch007,
    #[allow(dead_code)]
    SoundsUiTick001,
    #[allow(dead_code)]
    SoundsUiTick002,
    #[allow(dead_code)]
    SoundsUiTick004,
    #[allow(dead_code)]
    SoundsUiToggle001,
    #[allow(dead_code)]
    SoundsUiToggle002,
    #[allow(dead_code)]
    SoundsUiToggle003,
    #[allow(dead_code)]
    SoundsUiToggle004,
    #[allow(dead_code)]
    TextCredits,
}

impl FileAssets {
    pub fn path(&self) -> &'static str {
        match self {
            FileAssets::FontsEditundo => "fonts/editundo.ttf",
            FileAssets::ImagesGamePrimitivesRect => "images/game/primitives/rect.png",
            FileAssets::ImagesGameGameBackground => "images/game/game/background.jpg",
            FileAssets::ImagesGameMenuBackground => "images/game/menu/background.jpg",
            FileAssets::ImagesGameTilemapPacked => "images/game/tilemap_packed.png",
            FileAssets::ImagesGameUnitsTank => "images/game/units/tank.png",
            FileAssets::ImagesGameUnitsAntiair => "images/game/units/antiair.png",
            FileAssets::ImagesGameUnitsApc => "images/game/units/apc.png",
            FileAssets::ImagesGameUnitsArtillery => "images/game/units/artillery.png",
            FileAssets::ImagesGameUnitsBattlecruiser => "images/game/units/battlecruiser.png",
            FileAssets::ImagesGameUnitsBcopter => "images/game/units/bcopter.png",
            FileAssets::ImagesGameUnitsBomber => "images/game/units/bomber.png",
            FileAssets::ImagesGameUnitsCarrier => "images/game/units/carrier.png",
            FileAssets::ImagesGameUnitsCruiser => "images/game/units/cruiser.png",
            FileAssets::ImagesGameUnitsFighter => "images/game/units/fighter.png",
            FileAssets::ImagesGameUnitsInfantry => "images/game/units/infantry.png",
            FileAssets::ImagesGameUnitsLander => "images/game/units/lander.png",
            FileAssets::ImagesGameUnitsMaintank => "images/game/units/maintank.png",
            FileAssets::ImagesGameUnitsMech => "images/game/units/mech.png",
            FileAssets::ImagesGameUnitsMissile => "images/game/units/missile.png",
            FileAssets::ImagesGameUnitsRecon => "images/game/units/recon.png",
            FileAssets::ImagesGameUnitsRocketlauncher => "images/game/units/rocketlauncher.png",
            FileAssets::ImagesGameUnitsSub => "images/game/units/sub.png",
            FileAssets::ImagesGameUnitsSupplyship => "images/game/units/supplyship.png",
            FileAssets::ImagesGameUnitsTcopter => "images/game/units/tcopter.png",
            FileAssets::ImagesGameUnitsWartank => "images/game/units/wartank.png",
            FileAssets::ImagesGameTilemap2 => "images/game/tilemap2.png",
            FileAssets::ImagesGameEffectsExplosion => "images/game/effects/explosion.png",
            FileAssets::ImagesGameEffectsBomb => "images/game/effects/bomb.png",
            FileAssets::ImagesGameEffectsGunattack => "images/game/effects/gunattack.png",
            FileAssets::ImagesGameEffectsSmallExplosion => "images/game/effects/small_explosion.png",
            FileAssets::ImagesGameEffectsTorpedo => "images/game/effects/torpedo.png",
            FileAssets::ImagesGameEffectsVulcanCannon => "images/game/effects/vulcan_cannon.png",
            FileAssets::ImagesGameCursorHud36X36 => "images/game/cursor hud 36x36.png",
            FileAssets::ImagesGameCursorHud38X38 => "images/game/cursor hud 38x38.png",
            FileAssets::ImagesGameCursorAttack40X40 => "images/game/cursor_attack_40x40.png",
            FileAssets::ImagesGameCursorAttack42X42 => "images/game/cursor_attack_42x42.png",
            FileAssets::ImagesGameMovementHud => "images/game/movement hud.png",
            FileAssets::ImagesGameTerrain => "images/game/terrain.png",
            FileAssets::ImagesIconsNodesCarOutline => "images/icons/nodes/car-outline.svg",
            FileAssets::ImagesIconsNodesFlagOutline => "images/icons/nodes/flag-outline.svg",
            FileAssets::ImagesIconsNodesLayersOutline => "images/icons/nodes/layers-outline.svg",
            FileAssets::ImagesIconsNodesPaperPlaneOutline => "images/icons/nodes/paper-plane-outline.svg",
            FileAssets::ImagesIconsNodesPersonOutline => "images/icons/nodes/person-outline.svg",
            FileAssets::ImagesIconsNodesSettings2Outline => "images/icons/nodes/settings-2-outline.svg",
            FileAssets::ImagesIconsNodesDownloadOutline => "images/icons/nodes/download-outline.svg",
            FileAssets::ImagesIconsUiIconsDefault => "images/icons/ui/iconsDefault.png",
            FileAssets::ImagesIconsUiSheetBlack1X => "images/icons/ui/sheet_black1x.png",
            FileAssets::ImagesIconsUiSheetBlack2X => "images/icons/ui/sheet_black2x.png",
            FileAssets::ImagesIconsUiSheetWhite1X => "images/icons/ui/sheet_white1x.png",
            FileAssets::ImagesIconsUiSheetWhite2X => "images/icons/ui/sheet_white2x.png",
            FileAssets::ImagesThemeBlueSheet => "images/theme/blueSheet.png",
            FileAssets::ImagesThemeCursor => "images/theme/cursor.png",
            FileAssets::ImagesThemeGreenSheet => "images/theme/greenSheet.png",
            FileAssets::ImagesThemeGreySheet => "images/theme/greySheet.png",
            FileAssets::ImagesThemeRedSheet => "images/theme/redSheet.png",
            FileAssets::ImagesThemeYellowSheet => "images/theme/yellowSheet.png",
            FileAssets::MusicsCyberpunkMoonlightSonataV2 => "musics/Cyberpunk Moonlight Sonata v2.mp3",
            FileAssets::MusicsAwesomeness => "musics/awesomeness.wav",
            FileAssets::MusicsImmaculateCommandGeorgesDeMorgansThemeCodenameRecon => "musics/Immaculate_Command__Georges_DeMorgans_Theme_Codename_RECON.mp3",
            FileAssets::MusicsTempestassAceInTheHoleMinatusLeonusMacrosThemeUnfinishedV08 => "musics/Tempestass_Ace_in_the_Hole__Minatus_Leonus_Macros_Theme_UNFINISHED_v0.8.mp3",
            FileAssets::SoundsChipsHandle6 => "sounds/chipsHandle6.ogg",
            FileAssets::SoundsEngineCircular000 => "sounds/engineCircular_000.ogg",
            FileAssets::SoundsExplosionCrunch000 => "sounds/explosionCrunch_000.ogg",
            FileAssets::SoundsExplosionCrunch004 => "sounds/explosionCrunch_004.ogg",
            FileAssets::SoundsFootstepCarpet001 => "sounds/footstep_carpet_001.ogg",
            FileAssets::SoundsHold => "sounds/hold.ogg",
            FileAssets::SoundsImpactMining004 => "sounds/impactMining_004.ogg",
            FileAssets::SoundsImpactPlateMedium004 => "sounds/impactPlate_medium_004.ogg",
            FileAssets::SoundsLaserLarge004 => "sounds/laserLarge_004.ogg",
            FileAssets::SoundsLaserRetro003 => "sounds/laserRetro_003.ogg",
            FileAssets::SoundsLaserSmall001 => "sounds/laserSmall_001.ogg",
            FileAssets::SoundsSpaceEngine001 => "sounds/spaceEngine_001.ogg",
            FileAssets::SoundsUiBack001 => "sounds/ui/back_001.ogg",
            FileAssets::SoundsUiBack002 => "sounds/ui/back_002.ogg",
            FileAssets::SoundsUiBack003 => "sounds/ui/back_003.ogg",
            FileAssets::SoundsUiBack004 => "sounds/ui/back_004.ogg",
            FileAssets::SoundsUiBong001 => "sounds/ui/bong_001.ogg",
            FileAssets::SoundsUiClick001 => "sounds/ui/click_001.ogg",
            FileAssets::SoundsUiClick002 => "sounds/ui/click_002.ogg",
            FileAssets::SoundsUiClick003 => "sounds/ui/click_003.ogg",
            FileAssets::SoundsUiClick004 => "sounds/ui/click_004.ogg",
            FileAssets::SoundsUiClick005 => "sounds/ui/click_005.ogg",
            FileAssets::SoundsUiClose001 => "sounds/ui/close_001.ogg",
            FileAssets::SoundsUiClose002 => "sounds/ui/close_002.ogg",
            FileAssets::SoundsUiClose003 => "sounds/ui/close_003.ogg",
            FileAssets::SoundsUiClose004 => "sounds/ui/close_004.ogg",
            FileAssets::SoundsUiConfirmation001 => "sounds/ui/confirmation_001.ogg",
            FileAssets::SoundsUiConfirmation002 => "sounds/ui/confirmation_002.ogg",
            FileAssets::SoundsUiConfirmation003 => "sounds/ui/confirmation_003.ogg",
            FileAssets::SoundsUiConfirmation004 => "sounds/ui/confirmation_004.ogg",
            FileAssets::SoundsUiDoorOpen001 => "sounds/ui/doorOpen_001.ogg",
            FileAssets::SoundsUiDoorOpen002 => "sounds/ui/doorOpen_002.ogg",
            FileAssets::SoundsUiDrop001 => "sounds/ui/drop_001.ogg",
            FileAssets::SoundsUiDrop002 => "sounds/ui/drop_002.ogg",
            FileAssets::SoundsUiDrop003 => "sounds/ui/drop_003.ogg",
            FileAssets::SoundsUiDrop004 => "sounds/ui/drop_004.ogg",
            FileAssets::SoundsUiError001 => "sounds/ui/error_001.ogg",
            FileAssets::SoundsUiError002 => "sounds/ui/error_002.ogg",
            FileAssets::SoundsUiError003 => "sounds/ui/error_003.ogg",
            FileAssets::SoundsUiError004 => "sounds/ui/error_004.ogg",
            FileAssets::SoundsUiError005 => "sounds/ui/error_005.ogg",
            FileAssets::SoundsUiError006 => "sounds/ui/error_006.ogg",
            FileAssets::SoundsUiError007 => "sounds/ui/error_007.ogg",
            FileAssets::SoundsUiError008 => "sounds/ui/error_008.ogg",
            FileAssets::SoundsUiGlass001 => "sounds/ui/glass_001.ogg",
            FileAssets::SoundsUiGlass002 => "sounds/ui/glass_002.ogg",
            FileAssets::SoundsUiGlass003 => "sounds/ui/glass_003.ogg",
            FileAssets::SoundsUiGlass004 => "sounds/ui/glass_004.ogg",
            FileAssets::SoundsUiGlass005 => "sounds/ui/glass_005.ogg",
            FileAssets::SoundsUiGlass006 => "sounds/ui/glass_006.ogg",
            FileAssets::SoundsUiGlitch001 => "sounds/ui/glitch_001.ogg",
            FileAssets::SoundsUiGlitch002 => "sounds/ui/glitch_002.ogg",
            FileAssets::SoundsUiGlitch003 => "sounds/ui/glitch_003.ogg",
            FileAssets::SoundsUiGlitch004 => "sounds/ui/glitch_004.ogg",
            FileAssets::SoundsUiMaximize001 => "sounds/ui/maximize_001.ogg",
            FileAssets::SoundsUiMaximize002 => "sounds/ui/maximize_002.ogg",
            FileAssets::SoundsUiMaximize003 => "sounds/ui/maximize_003.ogg",
            FileAssets::SoundsUiMaximize004 => "sounds/ui/maximize_004.ogg",
            FileAssets::SoundsUiMaximize005 => "sounds/ui/maximize_005.ogg",
            FileAssets::SoundsUiMaximize006 => "sounds/ui/maximize_006.ogg",
            FileAssets::SoundsUiMaximize007 => "sounds/ui/maximize_007.ogg",
            FileAssets::SoundsUiMaximize008 => "sounds/ui/maximize_008.ogg",
            FileAssets::SoundsUiMaximize009 => "sounds/ui/maximize_009.ogg",
            FileAssets::SoundsUiMinimize001 => "sounds/ui/minimize_001.ogg",
            FileAssets::SoundsUiMinimize002 => "sounds/ui/minimize_002.ogg",
            FileAssets::SoundsUiMinimize003 => "sounds/ui/minimize_003.ogg",
            FileAssets::SoundsUiMinimize004 => "sounds/ui/minimize_004.ogg",
            FileAssets::SoundsUiMinimize005 => "sounds/ui/minimize_005.ogg",
            FileAssets::SoundsUiMinimize006 => "sounds/ui/minimize_006.ogg",
            FileAssets::SoundsUiMinimize007 => "sounds/ui/minimize_007.ogg",
            FileAssets::SoundsUiMinimize008 => "sounds/ui/minimize_008.ogg",
            FileAssets::SoundsUiMinimize009 => "sounds/ui/minimize_009.ogg",
            FileAssets::SoundsUiOpen001 => "sounds/ui/open_001.ogg",
            FileAssets::SoundsUiOpen002 => "sounds/ui/open_002.ogg",
            FileAssets::SoundsUiOpen003 => "sounds/ui/open_003.ogg",
            FileAssets::SoundsUiOpen004 => "sounds/ui/open_004.ogg",
            FileAssets::SoundsUiPluck001 => "sounds/ui/pluck_001.ogg",
            FileAssets::SoundsUiPluck002 => "sounds/ui/pluck_002.ogg",
            FileAssets::SoundsUiQuestion001 => "sounds/ui/question_001.ogg",
            FileAssets::SoundsUiQuestion002 => "sounds/ui/question_002.ogg",
            FileAssets::SoundsUiQuestion003 => "sounds/ui/question_003.ogg",
            FileAssets::SoundsUiQuestion004 => "sounds/ui/question_004.ogg",
            FileAssets::SoundsUiScratch001 => "sounds/ui/scratch_001.ogg",
            FileAssets::SoundsUiScratch002 => "sounds/ui/scratch_002.ogg",
            FileAssets::SoundsUiScratch003 => "sounds/ui/scratch_003.ogg",
            FileAssets::SoundsUiScratch004 => "sounds/ui/scratch_004.ogg",
            FileAssets::SoundsUiScratch005 => "sounds/ui/scratch_005.ogg",
            FileAssets::SoundsUiScroll001 => "sounds/ui/scroll_001.ogg",
            FileAssets::SoundsUiScroll002 => "sounds/ui/scroll_002.ogg",
            FileAssets::SoundsUiScroll003 => "sounds/ui/scroll_003.ogg",
            FileAssets::SoundsUiScroll004 => "sounds/ui/scroll_004.ogg",
            FileAssets::SoundsUiScroll005 => "sounds/ui/scroll_005.ogg",
            FileAssets::SoundsUiSelect001 => "sounds/ui/select_001.ogg",
            FileAssets::SoundsUiSelect002 => "sounds/ui/select_002.ogg",
            FileAssets::SoundsUiSelect003 => "sounds/ui/select_003.ogg",
            FileAssets::SoundsUiSelect004 => "sounds/ui/select_004.ogg",
            FileAssets::SoundsUiSelect005 => "sounds/ui/select_005.ogg",
            FileAssets::SoundsUiSelect006 => "sounds/ui/select_006.ogg",
            FileAssets::SoundsUiSelect007 => "sounds/ui/select_007.ogg",
            FileAssets::SoundsUiSelect008 => "sounds/ui/select_008.ogg",
            FileAssets::SoundsUiSwitch001 => "sounds/ui/switch_001.ogg",
            FileAssets::SoundsUiSwitch002 => "sounds/ui/switch_002.ogg",
            FileAssets::SoundsUiSwitch003 => "sounds/ui/switch_003.ogg",
            FileAssets::SoundsUiSwitch004 => "sounds/ui/switch_004.ogg",
            FileAssets::SoundsUiSwitch005 => "sounds/ui/switch_005.ogg",
            FileAssets::SoundsUiSwitch006 => "sounds/ui/switch_006.ogg",
            FileAssets::SoundsUiSwitch007 => "sounds/ui/switch_007.ogg",
            FileAssets::SoundsUiTick001 => "sounds/ui/tick_001.ogg",
            FileAssets::SoundsUiTick002 => "sounds/ui/tick_002.ogg",
            FileAssets::SoundsUiTick004 => "sounds/ui/tick_004.ogg",
            FileAssets::SoundsUiToggle001 => "sounds/ui/toggle_001.ogg",
            FileAssets::SoundsUiToggle002 => "sounds/ui/toggle_002.ogg",
            FileAssets::SoundsUiToggle003 => "sounds/ui/toggle_003.ogg",
            FileAssets::SoundsUiToggle004 => "sounds/ui/toggle_004.ogg",
            FileAssets::TextCredits => "text/credits.md",
        }
    }

    #[allow(dead_code)]
    pub fn scene(&self, scene_nr: i32) -> String {
        format!("{}#Scene{scene_nr}", self.path())
    }

    #[allow(dead_code)]
    pub fn load<'a, A: Asset>(&self, assets: &Res<AssetServer>) -> Handle<A> {
        assets.load(self.path())
    }
}
