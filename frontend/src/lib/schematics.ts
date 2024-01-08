import { deserialize } from "@xmcl/nbt";

export async function getSchematicData(data: Uint8Array) {
  const schematic = await deserialize(data);
  console.log(schematic);
  // const blocks: {
  //   nbt?: { value: any };
  //   pos: { value: { value: [number, number, number] } };
  //   state: { value: number };
  // }[] = schematic.value.blocks?.value.value;

  // const blockList = new Map<string, number>();
  // const mods = new Set<string>();
  // for (const block of blocks) {
  //   if (!block.nbt?.value.id) continue;
  //   mods.add(block.nbt.value.id.value.split(":")[0]);
  //   blockList.set(
  //     block.nbt.value.id.value,
  //     (blockList.get(block.nbt.value.id.value) ?? 0) + 1
  //   );
  // }
  // return { blockList, mods };
}
