export const SEARCH_TEXT_STORAGE_KEY = "index.searchText";

function getStorage(storage) {
  if (storage) {
    return storage;
  }

  return typeof localStorage === "undefined" ? null : localStorage;
}

export function loadSearchText(storage) {
  try {
    return getStorage(storage)?.getItem(SEARCH_TEXT_STORAGE_KEY) ?? "";
  } catch {
    return "";
  }
}

export function saveSearchText(searchText, storage) {
  try {
    getStorage(storage)?.setItem(SEARCH_TEXT_STORAGE_KEY, searchText ?? "");
  } catch {
    // Search is still usable when persistent storage is unavailable.
  }
}

export function filterZipList(zipList, searchText) {
  const keyword = String(searchText ?? "").trim().toLocaleLowerCase();

  if (keyword === "") {
    return zipList;
  }

  return zipList.filter((item) => String(item.name ?? "").toLocaleLowerCase().includes(keyword));
}
