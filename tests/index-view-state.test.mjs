import assert from "node:assert/strict";
import test from "node:test";

import {
  filterZipList,
  loadSearchText,
  saveSearchText,
} from "../src/utils/index-view-state.js";

function createMemoryStorage() {
  const values = new Map();

  return {
    getItem(key) {
      return values.get(key) ?? null;
    },
    setItem(key, value) {
      values.set(key, String(value));
    },
  };
}

test("search text survives index view recreation", () => {
  const storage = createMemoryStorage();

  saveSearchText("  火影  ", storage);

  assert.equal(loadSearchText(storage), "  火影  ");
});

test("filter keeps only filenames containing the search text", () => {
  const files = [
    {name: "火影忍者.zip"},
    {name: "海贼王.zip"},
    {name: "火影外传.zip"},
  ];

  assert.deepEqual(
    filterZipList(files, "影外").map((item) => item.name),
    ["火影外传.zip"],
  );
});
