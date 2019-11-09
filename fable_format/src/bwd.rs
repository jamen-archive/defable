pub mod decode;
pub mod encode;

// Bwd is a compiled Wld. A description is not contained the Wiki, however there is one in the forums <http://fabletlcmod.com/forum/index.php?t=msg&goto=39700&&srch=bwd#msg_39700>. Its pasted here for posterity.
//
// Map Data Header
// [4] bytes   - Number of map entries + 1
//
// Map Data Entries
// [4] bytes   - LevelName string length
// ~string     - LevelName
// [4] bytes   - LevelScriptName string length
// ~string     - LevelScriptName
// [1] byte    - Boolean? Always true
// [1] byte    - LoadedOnPlayerProximity boolean
// [1] byte    - IsSea boolean
// [4] bytes   - Map X start
// [4] bytes   - Map X end
// [4] bytes   - Map Y start
// [4] bytes   - Map Y end
// [1] byte    - Boolean? Always true
// [4] bytes   - Map UID
// [4] bytes   - Null, end of entry
//
// Region Data Header
// [4] bytes   - Number of region entries + 1
//
// Region Data Entries
// [4] bytes   - NumberOfContainedMaps
// [4] bytes   - NumberOfSeenMaps
//   [4] bytes - ContainedMap
//   (Repeat for x number of entries, omit if 0)
//   [4] bytes - SeenMap
//   (Repeat for x number of entries, omit if 0)
// [4] bytes   - RegionName string length
// ~string     - RegionName
// [4] bytes   - NewDisplayName string length
// ~string     - NewDisplayName
// [4] bytes   - RegionDef string length
// ~string     - RegionDef
// [4] bytes   - MiniMapGraphic string length
// ~string     - MiniMapGraphic
// [1] byte    - AppearOnWorldMap boolean
// [1] byte    - Boolean? Always true
// [1] byte    - Boolean? Always true
// [4] bytes   - Float, MiniMapScale
// [4] bytes   - Signed, MiniMapOffsetX
// [4] bytes   - Signed, MiniMapOffsetY
// [4] bytes   - Signed, WorldMapOffsetX
// [4] bytes   - Signed, WorldMapOffsetY
// [4] bytes   - MiniMapRegionExitTextOffset entries
//   [4] bytes - MiniMapRegionExitText string length
//   ~string   - MiniMapRegionExitText
//   [4] bytes - MiniMapRegionExitTextOffsetX
//   [4] bytes - MiniMapRegionExitTextOffsetY
//   (Repeat for x number of entries, omit if 0)