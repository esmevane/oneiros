import * as Generated from "./generated";

// ── URL templates ──────────────────────────────────────────────────────────

export const routes = {
  get: {
    actors: "/actors",
    actors_by_id: "/actors/:id",
    agents: "/agents",
    agents_by_name: "/agents/:name",
    bookmarks: "/bookmarks",
    cognitions: "/cognitions",
    cognitions_by_id: "/cognitions/:id",
    connections: "/connections",
    connections_by_id: "/connections/:id",
    continuity: "/continuity",
    continuity_guidebook: "/continuity/:agent/guidebook",
    experiences: "/experiences",
    experiences_by_id: "/experiences/:id",
    follows: "/follows",
    follows_by_id: "/follows/:id",
    health: "/health",
    levels: "/levels",
    levels_by_name: "/levels/:name",
    memories: "/memories",
    memories_by_id: "/memories/:id",
    natures: "/natures",
    natures_by_name: "/natures/:name",
    peers: "/peers",
    peers_by_id: "/peers/:id",
    personas: "/personas",
    personas_by_name: "/personas/:name",
    pressures: "/pressures",
    pressures_by_agent: "/pressures/:agent",
    projects: "/projects",
    projects_by_name: "/projects/:name",
    search: "/search",
    slices: "/slices",
    sensations: "/sensations",
    sensations_by_name: "/sensations/:name",
    storage: "/storage",
    storage_by_ref_key: "/storage/:ref_key",
    tenants: "/tenants",
    tenants_by_id: "/tenants/:id",
    textures: "/textures",
    textures_by_name: "/textures/:name",
    tickets: "/tickets",
    tickets_by_id: "/tickets/:id",
    trail_from: "/trail/from/:event_id",
    trail_of: "/trail/of/:ref",
    urges: "/urges",
    urges_by_name: "/urges/:name",
  },
  post: {
    actors: "/actors",
    agents: "/agents",
    bookmarks: "/bookmarks",
    bookmarks_collect: "/bookmarks/collect",
    bookmarks_follow: "/bookmarks/follow",
    bookmarks_merge: "/bookmarks/merge",
    bookmarks_share: "/bookmarks/share",
    bookmarks_switch: "/bookmarks/switch",
    bookmarks_unfollow: "/bookmarks/unfollow",
    cognitions: "/cognitions",
    connections: "/connections",
    continuity_dream: "/continuity/:agent/dream",
    continuity_emerge: "/continuity",
    continuity_introspect: "/continuity/:agent/introspect",
    continuity_reflect: "/continuity/:agent/reflect",
    continuity_sense: "/continuity/:agent/sense",
    continuity_sleep: "/continuity/:agent/sleep",
    continuity_wake: "/continuity/:agent/wake",
    experiences: "/experiences",
    host_init: "/host",
    memories: "/memories",
    peers: "/peers",
    projects: "/projects",
    seed_agents: "/seed/agents",
    seed_core: "/seed/core",
    slices: "/slices",
    slices_diff: "/slices/diff",
    storage: "/storage",
    tenants: "/tenants",
    tickets: "/tickets",
    tickets_validate: "/tickets/validate",
  },
  put: {
    agents_by_name: "/agents/:name",
    levels_by_name: "/levels/:name",
    natures_by_name: "/natures/:name",
    personas_by_name: "/personas/:name",
    sensations_by_name: "/sensations/:name",
    textures_by_name: "/textures/:name",
    urges_by_name: "/urges/:name",
  },
  delete: {
    agents_by_name: "/agents/:name",
    connections_by_id: "/connections/:id",
    continuity_recede: "/continuity/:agent",
    levels_by_name: "/levels/:name",
    natures_by_name: "/natures/:name",
    peers_by_id: "/peers/:id",
    personas_by_name: "/personas/:name",
    sensations_by_name: "/sensations/:name",
    slices_by_name: "/slices/:name",
    storage_by_ref_key: "/storage/:ref_key",
    textures_by_name: "/textures/:name",
    urges_by_name: "/urges/:name",
  },
} as const;

// ── API functions ──────────────────────────────────────────────────────────

const throwOff = { throwOnError: false as const };

export const api = {
  get: {
    actors_by_id: (opts: Parameters<typeof Generated.getActor>[0]) =>
      call(Generated.getActor({ ...opts, ...throwOff })),
    agents_by_name: (opts: Parameters<typeof Generated.getAgent>[0]) =>
      call(Generated.getAgent({ ...opts, ...throwOff })),
    cognitions_by_id: (opts: Parameters<typeof Generated.getCognition>[0]) =>
      call(Generated.getCognition({ ...opts, ...throwOff })),
    connections_by_id: (opts: Parameters<typeof Generated.getConnection>[0]) =>
      call(Generated.getConnection({ ...opts, ...throwOff })),
    experiences_by_id: (opts: Parameters<typeof Generated.getExperience>[0]) =>
      call(Generated.getExperience({ ...opts, ...throwOff })),
    follows_by_id: (opts: Parameters<typeof Generated.getFollow>[0]) =>
      call(Generated.getFollow({ ...opts, ...throwOff })),
    health: (opts?: Parameters<typeof Generated.getHealth>[0]) =>
      call(Generated.getHealth({ ...opts, ...throwOff })),
    levels: (opts?: Parameters<typeof Generated.listLevels>[0]) =>
      call(Generated.listLevels({ ...opts, ...throwOff })),
    memories: (opts?: Parameters<typeof Generated.listMemories>[0]) =>
      call(Generated.listMemories({ ...opts, ...throwOff })),
    natures: (opts?: Parameters<typeof Generated.listNatures>[0]) =>
      call(Generated.listNatures({ ...opts, ...throwOff })),
    peers: (opts?: Parameters<typeof Generated.listPeers>[0]) =>
      call(Generated.listPeers({ ...opts, ...throwOff })),
    personas: (opts?: Parameters<typeof Generated.listPersonas>[0]) =>
      call(Generated.listPersonas({ ...opts, ...throwOff })),
    pressures: (opts?: Parameters<typeof Generated.listPressure>[0]) =>
      call(Generated.listPressure({ ...opts, ...throwOff })),
    projects: (opts?: Parameters<typeof Generated.listProjects>[0]) =>
      call(Generated.listProjects({ ...opts, ...throwOff })),
    sensations: (opts?: Parameters<typeof Generated.listSensations>[0]) =>
      call(Generated.listSensations({ ...opts, ...throwOff })),
    tenants: (opts?: Parameters<typeof Generated.listTenants>[0]) =>
      call(Generated.listTenants({ ...opts, ...throwOff })),
    textures: (opts?: Parameters<typeof Generated.listTextures>[0]) =>
      call(Generated.listTextures({ ...opts, ...throwOff })),
    tickets: (opts?: Parameters<typeof Generated.listTickets>[0]) =>
      call(Generated.listTickets({ ...opts, ...throwOff })),
    urges: (opts?: Parameters<typeof Generated.listUrges>[0]) =>
      call(Generated.listUrges({ ...opts, ...throwOff })),
    search: (opts?: Parameters<typeof Generated.search>[0]) =>
      call(Generated.search({ ...opts, ...throwOff })),
    slices: (opts?: Parameters<typeof Generated.listSlices>[0]) =>
      call(Generated.listSlices({ ...opts, ...throwOff })),
    storage_by_ref_key: (opts: Parameters<typeof Generated.showBlob>[0]) =>
      call(Generated.showBlob({ ...opts, ...throwOff })),
    personas_by_name: (opts: Parameters<typeof Generated.showPersona>[0]) =>
      call(Generated.showPersona({ ...opts, ...throwOff })),
    sensations_by_name: (opts: Parameters<typeof Generated.showSensation>[0]) =>
      call(Generated.showSensation({ ...opts, ...throwOff })),
    tenants_by_id: (opts: Parameters<typeof Generated.showTenant>[0]) =>
      call(Generated.showTenant({ ...opts, ...throwOff })),
    textures_by_name: (opts: Parameters<typeof Generated.showTexture>[0]) =>
      call(Generated.showTexture({ ...opts, ...throwOff })),
    tickets_by_id: (opts: Parameters<typeof Generated.showTicket>[0]) =>
      call(Generated.showTicket({ ...opts, ...throwOff })),
    urges_by_name: (opts: Parameters<typeof Generated.showUrge>[0]) =>
      call(Generated.showUrge({ ...opts, ...throwOff })),
    continuity: (opts?: Parameters<typeof Generated.statusAgent>[0]) =>
      call(Generated.statusAgent({ ...opts, ...throwOff })),
    trail_from: (opts?: Parameters<typeof Generated.trailFrom>[0]) =>
      call(Generated.trailFrom({ ...opts, ...throwOff })),
    trail_of: (opts?: Parameters<typeof Generated.trailOf>[0]) =>
      call(Generated.trailOf({ ...opts, ...throwOff })),
  },
  post: {
    cognitions: (opts: Parameters<typeof Generated.addCognition>[0]) =>
      call(Generated.addCognition({ ...opts, ...throwOff })),
    memories: (opts: Parameters<typeof Generated.addMemory>[0]) =>
      call(Generated.addMemory({ ...opts, ...throwOff })),
    peers: (opts: Parameters<typeof Generated.addPeer>[0]) =>
      call(Generated.addPeer({ ...opts, ...throwOff })),
    bookmarks_collect: (
      opts: Parameters<typeof Generated.collectBookmark>[0],
    ) => call(Generated.collectBookmark({ ...opts, ...throwOff })),
    actors: (opts: Parameters<typeof Generated.createActor>[0]) =>
      call(Generated.createActor({ ...opts, ...throwOff })),
    agents: (opts: Parameters<typeof Generated.createAgent>[0]) =>
      call(Generated.createAgent({ ...opts, ...throwOff })),
    bookmarks: (opts: Parameters<typeof Generated.createBookmark>[0]) =>
      call(Generated.createBookmark({ ...opts, ...throwOff })),
    connections: (opts: Parameters<typeof Generated.createConnection>[0]) =>
      call(Generated.createConnection({ ...opts, ...throwOff })),
    experiences: (opts: Parameters<typeof Generated.createExperience>[0]) =>
      call(Generated.createExperience({ ...opts, ...throwOff })),
    projects: (opts: Parameters<typeof Generated.createProject>[0]) =>
      call(Generated.createProject({ ...opts, ...throwOff })),
    tenants: (opts: Parameters<typeof Generated.createTenant>[0]) =>
      call(Generated.createTenant({ ...opts, ...throwOff })),
    tickets: (opts: Parameters<typeof Generated.createTicket>[0]) =>
      call(Generated.createTicket({ ...opts, ...throwOff })),
    continuity_dream: (opts: Parameters<typeof Generated.dreamAgent>[0]) =>
      call(Generated.dreamAgent({ ...opts, ...throwOff })),
    continuity_emerge: (opts: Parameters<typeof Generated.emergeAgent>[0]) =>
      call(Generated.emergeAgent({ ...opts, ...throwOff })),
    bookmarks_follow: (opts: Parameters<typeof Generated.followBookmark>[0]) =>
      call(Generated.followBookmark({ ...opts, ...throwOff })),
    host_init: (opts: Parameters<typeof Generated.initHost>[0]) =>
      call(Generated.initHost({ ...opts, ...throwOff })),
    continuity_introspect: (
      opts: Parameters<typeof Generated.introspectAgent>[0],
    ) => call(Generated.introspectAgent({ ...opts, ...throwOff })),
    bookmarks_merge: (opts: Parameters<typeof Generated.mergeBookmark>[0]) =>
      call(Generated.mergeBookmark({ ...opts, ...throwOff })),
    continuity_reflect: (opts: Parameters<typeof Generated.reflectAgent>[0]) =>
      call(Generated.reflectAgent({ ...opts, ...throwOff })),
    seed_agents: (opts?: Parameters<typeof Generated.seedAgents>[0]) =>
      call(Generated.seedAgents({ ...opts, ...throwOff })),
    seed_core: (opts?: Parameters<typeof Generated.seedCore>[0]) =>
      call(Generated.seedCore({ ...opts, ...throwOff })),
    slices: (opts: Parameters<typeof Generated.createSlice>[0]) =>
      call(Generated.createSlice({ ...opts, ...throwOff })),
    slices_diff: (opts: Parameters<typeof Generated.diffSlices>[0]) =>
      call(Generated.diffSlices({ ...opts, ...throwOff })),
    continuity_sense: (opts: Parameters<typeof Generated.senseAgent>[0]) =>
      call(Generated.senseAgent({ ...opts, ...throwOff })),
    bookmarks_share: (opts: Parameters<typeof Generated.shareBookmark>[0]) =>
      call(Generated.shareBookmark({ ...opts, ...throwOff })),
    continuity_sleep: (opts: Parameters<typeof Generated.sleepAgent>[0]) =>
      call(Generated.sleepAgent({ ...opts, ...throwOff })),
    bookmarks_switch: (opts: Parameters<typeof Generated.switchBookmark>[0]) =>
      call(Generated.switchBookmark({ ...opts, ...throwOff })),
    bookmarks_unfollow: (
      opts: Parameters<typeof Generated.unfollowBookmark>[0],
    ) => call(Generated.unfollowBookmark({ ...opts, ...throwOff })),
    storage: (opts: Parameters<typeof Generated.uploadBlob>[0]) =>
      call(Generated.uploadBlob({ ...opts, ...throwOff })),
    tickets_validate: (opts: Parameters<typeof Generated.validateTicket>[0]) =>
      call(Generated.validateTicket({ ...opts, ...throwOff })),
    continuity_wake: (opts: Parameters<typeof Generated.wakeAgent>[0]) =>
      call(Generated.wakeAgent({ ...opts, ...throwOff })),
  },
  put: {
    levels_by_name: (opts: Parameters<typeof Generated.setLevel>[0]) =>
      call(Generated.setLevel({ ...opts, ...throwOff })),
    natures_by_name: (opts: Parameters<typeof Generated.setNature>[0]) =>
      call(Generated.setNature({ ...opts, ...throwOff })),
    personas_by_name: (opts: Parameters<typeof Generated.setPersona>[0]) =>
      call(Generated.setPersona({ ...opts, ...throwOff })),
    sensations_by_name: (opts: Parameters<typeof Generated.setSensation>[0]) =>
      call(Generated.setSensation({ ...opts, ...throwOff })),
    textures_by_name: (opts: Parameters<typeof Generated.setTexture>[0]) =>
      call(Generated.setTexture({ ...opts, ...throwOff })),
    urges_by_name: (opts: Parameters<typeof Generated.setUrge>[0]) =>
      call(Generated.setUrge({ ...opts, ...throwOff })),
    agents_by_name: (opts: Parameters<typeof Generated.updateAgent>[0]) =>
      call(Generated.updateAgent({ ...opts, ...throwOff })),
  },
  delete: {
    continuity_recede: (opts: Parameters<typeof Generated.recedeAgent>[0]) =>
      call(Generated.recedeAgent({ ...opts, ...throwOff })),
    agents_by_name: (opts: Parameters<typeof Generated.removeAgent>[0]) =>
      call(Generated.removeAgent({ ...opts, ...throwOff })),
    storage_by_ref_key: (opts: Parameters<typeof Generated.removeBlob>[0]) =>
      call(Generated.removeBlob({ ...opts, ...throwOff })),
    connections_by_id: (
      opts?: Parameters<typeof Generated.removeConnection>[0],
    ) => call(Generated.removeConnection({ ...opts, ...throwOff })),
    levels_by_name: (opts: Parameters<typeof Generated.removeLevel>[0]) =>
      call(Generated.removeLevel({ ...opts, ...throwOff })),
    natures_by_name: (opts: Parameters<typeof Generated.removeNature>[0]) =>
      call(Generated.removeNature({ ...opts, ...throwOff })),
    peers_by_id: (opts?: Parameters<typeof Generated.removePeer>[0]) =>
      call(Generated.removePeer({ ...opts, ...throwOff })),
    personas_by_name: (opts: Parameters<typeof Generated.removePersona>[0]) =>
      call(Generated.removePersona({ ...opts, ...throwOff })),
    sensations_by_name: (
      opts: Parameters<typeof Generated.removeSensation>[0],
    ) => call(Generated.removeSensation({ ...opts, ...throwOff })),
    slices_by_name: (opts?: Parameters<typeof Generated.deleteSlice>[0]) =>
      call(Generated.deleteSlice({ ...opts, ...throwOff })),
    textures_by_name: (opts: Parameters<typeof Generated.removeTexture>[0]) =>
      call(Generated.removeTexture({ ...opts, ...throwOff })),
    urges_by_name: (opts: Parameters<typeof Generated.removeUrge>[0]) =>
      call(Generated.removeUrge({ ...opts, ...throwOff })),
  },
} as const;

// ── Helpers ────────────────────────────────────────────────────────────────

class RequestProblem extends Error {
  constructor(
    message: string,
    public readonly response: Response,
    public readonly status: number,
  ) {
    super(message);
    this.name = "RequestProblem";
  }
}

export async function call<Data = unknown>(
  promise: Promise<{
    data?: Data;
    error?: unknown;
    request: Request;
    response: Response;
  }>,
): Promise<Data> {
  const result = await promise;

  if (typeof result.data !== "undefined") {
    return result.data as Data;
  }

  const detail =
    result.error instanceof Error
      ? result.error.message
      : typeof result.error === "string"
        ? result.error
        : JSON.stringify(result.error);

  throw new RequestProblem(detail, result.response, result.response.status);
}

// ── Re-exports ─────────────────────────────────────────────────────────────

export { client } from "./generated/client.gen";
export type * from "./generated/types.gen";

// ── Type guard: exhaustiveness + URL enforcement ───────────────────────────
//
// When the API changes and the client types are regenerated:
//   1. If a new endpoint appears → MappedFnNames fails (add the name)
//   2. If an endpoint is removed  → api.* referencing it breaks
//   3. If a URL or param changes  → routes.* breaks against Data.url
//
// Maintenance: one line per generated function in MappedFnNames, plus
// the corresponding wrappers in `api` and `routes`.

/** Every exported function from the generated SDK. */
type GeneratedFnNames = {
  [K in keyof typeof Generated]: (typeof Generated)[K] extends (
    ...args: any[]
  ) => any
    ? K
    : never;
}[keyof typeof Generated];

/** Extract the Data shape from a generated function's Options parameter. */
type DataOf<Fn> = Fn extends (options?: infer Opts) => any
  ? Opts extends { url: infer Url }
    ? { url: Url; path: Opts extends { path: infer P } ? P : never }
    : never
  : never;

/** Every generated function name must appear here. */
type MappedFnNames =
  | "addCognition"
  | "addMemory"
  | "addPeer"
  | "collectBookmark"
  | "createActor"
  | "createAgent"
  | "createBookmark"
  | "createConnection"
  | "createExperience"
  | "createProject"
  | "createSlice"
  | "createTenant"
  | "createTicket"
  | "deleteSlice"
  | "diffSlices"
  | "dreamAgent"
  | "emergeAgent"
  | "followBookmark"
  | "getActor"
  | "getAgent"
  | "getCognition"
  | "getConnection"
  | "getExperience"
  | "getFollow"
  | "getHealth"
  | "getLevel"
  | "getMemory"
  | "getNature"
  | "getPeer"
  | "getPressure"
  | "getProject"
  | "guidebookAgent"
  | "initHost"
  | "introspectAgent"
  | "listActors"
  | "listAgents"
  | "listBlobs"
  | "listBookmarks"
  | "listCognitions"
  | "listConnections"
  | "listExperiences"
  | "listFollows"
  | "listLevels"
  | "listMemories"
  | "listNatures"
  | "listPeers"
  | "listPersonas"
  | "listPressure"
  | "listProjects"
  | "listSensations"
  | "listSlices"
  | "listTenants"
  | "listTextures"
  | "listTickets"
  | "listUrges"
  | "mergeBookmark"
  | "recedeAgent"
  | "reflectAgent"
  | "removeAgent"
  | "removeBlob"
  | "removeConnection"
  | "removeLevel"
  | "removeNature"
  | "removePeer"
  | "removePersona"
  | "removeSensation"
  | "removeTexture"
  | "removeUrge"
  | "search"
  | "seedAgents"
  | "seedCore"
  | "senseAgent"
  | "setLevel"
  | "setNature"
  | "setPersona"
  | "setSensation"
  | "setTexture"
  | "setUrge"
  | "shareBookmark"
  | "showBlob"
  | "showPersona"
  | "showSensation"
  | "showTenant"
  | "showTexture"
  | "showTicket"
  | "showUrge"
  | "sleepAgent"
  | "statusAgent"
  | "switchBookmark"
  | "trailFrom"
  | "trailOf"
  | "unfollowBookmark"
  | "updateAgent"
  | "uploadBlob"
  | "validateTicket"
  | "wakeAgent";

/** Fail if the generated SDK has functions we haven't mapped. */
type _AssertAllMapped =
  Exclude<GeneratedFnNames, MappedFnNames> extends never
    ? true
    : [
        "❌ Unmapped API functions — add to MappedFnNames:",
        Exclude<GeneratedFnNames, MappedFnNames>,
      ];

/** Every URL that exists in the generated API. */
type GeneratedUrls = {
  [K in MappedFnNames]: DataOf<(typeof Generated)[K]> extends { url: infer U }
    ? U
    : never;
}[MappedFnNames];

/** Every route URL we've hand-authored. */
type RouteUrls =
  (typeof routes)[keyof typeof routes][keyof (typeof routes)[keyof typeof routes]];

/** Fail if any route URL doesn't match a generated endpoint. */
type _AssertRoutesValid =
  Exclude<RouteUrls, GeneratedUrls> extends never
    ? true
    : [
        "❌ Unknown route URL — check against generated types:",
        Exclude<RouteUrls, GeneratedUrls>,
      ];

/** Constraint gate — produces a type error when T is not `true`. */
type AssertTrue<T extends true> = T;

// Force TypeScript to evaluate the hole-driven-development assertions
type __assert_all_mapped = AssertTrue<_AssertAllMapped>;
type __assert_routes_valid = AssertTrue<_AssertRoutesValid>;
