/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */


/** OneOf type helpers */
type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };
type XOR<T, U> = (T | U) extends object ? (Without<T, U> & U) | (Without<U, T> & T) : T | U;
type OneOf<T extends any[]> = T extends [infer Only] ? Only : T extends [infer A, infer B, ...infer Rest] ? OneOf<[XOR<A, B>, ...Rest]> : never;

export interface paths {
  "/beta/api/alice": {
    get: operations["alice_route"];
  };
  "/beta/api/autosuggest": {
    post: operations["route"];
  };
  "/beta/api/explore/export": {
    post: operations["explore_export_optic"];
  };
  "/beta/api/fact_check": {
    post: operations["fact_check_route"];
  };
  "/beta/api/search": {
    post: operations["api"];
  };
  "/beta/api/sites/export": {
    post: operations["sites_export_optic"];
  };
  "/beta/api/summarize": {
    get: operations["summarize_route"];
  };
  "/beta/api/webgraph/knows_site": {
    post: operations["knows_site"];
  };
  "/beta/api/webgraph/similar_sites": {
    post: operations["similar_sites"];
  };
}

export type webhooks = Record<string, never>;

export interface components {
  schemas: {
    /**
     * SearchQuery
     * @example {
     *   "query": "hello world"
     * }
     */
    ApiSearchQuery: {
      flattenResponse?: boolean;
      numResults?: number | null;
      optic?: string | null;
      page?: number | null;
      query: string;
      returnRankingSignals?: boolean;
      safeSearch?: boolean | null;
      selectedRegion?: components["schemas"]["Region"] | null;
      siteRankings?: components["schemas"]["SiteRankings"] | null;
    };
    ApiSearchResult: (components["schemas"]["WebsitesResult"] & {
      /** @enum {string} */
      type: "websites";
    }) | (components["schemas"]["BangHit"] & {
      /** @enum {string} */
      type: "bang";
    });
    Bang: {
      c?: string | null;
      d?: string | null;
      r?: number | null;
      s?: string | null;
      sc?: string | null;
      t: string;
      u: string;
    };
    BangHit: {
      bang: components["schemas"]["Bang"];
      redirectTo: components["schemas"]["UrlWrapper"];
    };
    Calculation: {
      expr: components["schemas"]["Expr"];
      input: string;
      /** Format: double */
      result: number;
    };
    CodeOrText: {
      /** @enum {string} */
      type: "code";
      value: string;
    } | {
      /** @enum {string} */
      type: "text";
      value: string;
    };
    DisplayedAnswer: {
      answer: string;
      prettyUrl: string;
      snippet: string;
      title: string;
      url: string;
    };
    DisplayedEntity: {
      imageBase64?: string | null;
      info: ((string & string)[])[];
      /** Format: float */
      matchScore: number;
      relatedEntities: components["schemas"]["DisplayedEntity"][];
      smallAbstract: string;
      title: string;
    };
    DisplayedWebpage: {
      domain: string;
      prettyUrl: string;
      rankingSignals?: {
        [key: string]: components["schemas"]["SignalScore"];
      } | null;
      site: string;
      snippet: components["schemas"]["Snippet"];
      title: string;
      url: string;
    };
    /** @description base64 encoded `EncryptedState` */
    EncodedEncryptedState: string;
    EncodedSavedState: string;
    ExecutionState: {
      query: string;
      /** @enum {string} */
      type: "beginSearch";
    } | {
      query: string;
      result: components["schemas"]["SimplifiedWebsite"][];
      /** @enum {string} */
      type: "searchResult";
    } | {
      text: string;
      /** @enum {string} */
      type: "speaking";
    } | {
      state: components["schemas"]["EncodedEncryptedState"];
      /** @enum {string} */
      type: "done";
    };
    ExploreExportOpticParams: {
      chosenSites: string[];
      similarSites: string[];
    };
    Expr: OneOf<[{
      /** Format: double */
      Number: number;
    }, {
      Op: Record<string, never>[];
    }]>;
    FactCheckParams: {
      claim: string;
      evidence: string;
    };
    FactCheckResponse: {
      /** Format: double */
      score: number;
    };
    HighlightedSpellCorrection: {
      highlighted: string;
      raw: string;
    };
    KnowsSite: {
      site: string;
      /** @enum {string} */
      type: "known";
    } | {
      /** @enum {string} */
      type: "unknown";
    };
    /** @enum {string} */
    Region: "All" | "Denmark" | "France" | "Germany" | "Spain" | "US";
    ScoredSite: {
      description?: string | null;
      /** Format: double */
      score: number;
      site: string;
    };
    Sidebar: {
      /** @enum {string} */
      type: "entity";
      value: components["schemas"]["DisplayedEntity"];
    } | {
      /** @enum {string} */
      type: "stackOverflow";
      value: {
        answer: components["schemas"]["StackOverflowAnswer"];
        title: string;
      };
    };
    SignalScore: {
      /** Format: double */
      coefficient: number;
      /** Format: double */
      value: number;
    };
    SimilarSitesParams: {
      sites: string[];
      topN: number;
    };
    SimplifiedWebsite: {
      site: string;
      text: string;
      title: string;
      url: string;
    };
    SiteRankings: {
      blocked: string[];
      disliked: string[];
      liked: string[];
    };
    SitesExportOpticParams: {
      siteRankings: components["schemas"]["SiteRankings"];
    };
    Snippet: ({
      date?: string | null;
      text: components["schemas"]["TextSnippet"];
      /** @enum {string} */
      type: "normal";
    }) | {
      answers: components["schemas"]["StackOverflowAnswer"][];
      question: components["schemas"]["StackOverflowQuestion"];
      /** @enum {string} */
      type: "stackOverflowQA";
    };
    StackOverflowAnswer: {
      accepted: boolean;
      body: components["schemas"]["CodeOrText"][];
      date: string;
      /** Format: int32 */
      upvotes: number;
      url: string;
    };
    StackOverflowQuestion: {
      body: components["schemas"]["CodeOrText"][];
    };
    Suggestion: {
      highlighted: string;
      raw: string;
    };
    TextSnippet: {
      fragments: components["schemas"]["TextSnippetFragment"][];
    };
    TextSnippetFragment: {
      kind: components["schemas"]["TextSnippetFragmentKind"];
      text: string;
    };
    /** @enum {string} */
    TextSnippetFragmentKind: "normal" | "highlighted";
    /**
     * Url
     * @description Wrapper around `Url` that implements `ToSchema` for `Url`.
     */
    UrlWrapper: string;
    WebsitesResult: {
      directAnswer?: components["schemas"]["DisplayedAnswer"] | null;
      discussions?: components["schemas"]["DisplayedWebpage"][] | null;
      hasMoreResults: boolean;
      numHits: number;
      searchDurationMs: number;
      sidebar?: components["schemas"]["Sidebar"] | null;
      spellCorrectedQuery?: components["schemas"]["HighlightedSpellCorrection"] | null;
      webpages: components["schemas"]["DisplayedWebpage"][];
      widget?: components["schemas"]["Widget"] | null;
    };
    Widget: {
      /** @enum {string} */
      type: "calculator";
      value: components["schemas"]["Calculation"];
    };
  };
  responses: never;
  parameters: never;
  requestBodies: never;
  headers: never;
  pathItems: never;
}

export type $defs = Record<string, never>;

export type external = Record<string, never>;

export interface operations {

  alice_route: {
    parameters: {
      query: {
        message: string;
        optic?: string | null;
        prevState?: components["schemas"]["EncodedSavedState"] | null;
      };
    };
    responses: {
      /** @description Interact with Alice */
      200: {
        content: {
          "application/json": components["schemas"]["ExecutionState"];
        };
      };
    };
  };
  route: {
    parameters: {
      path: {
        q: string;
      };
    };
    responses: {
      /** @description Autosuggest */
      200: {
        content: {
          "application/json": components["schemas"]["Suggestion"][];
        };
      };
    };
  };
  explore_export_optic: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["ExploreExportOpticParams"];
      };
    };
    responses: {
      /** @description Export explored sites as an optic */
      200: {
        content: {
          "text/plain": string;
        };
      };
    };
  };
  fact_check_route: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["FactCheckParams"];
      };
    };
    responses: {
      /** @description Fact check the given claim against the given evidence */
      200: {
        content: {
          "application/json": components["schemas"]["FactCheckResponse"];
        };
      };
    };
  };
  api: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["ApiSearchQuery"];
      };
    };
    responses: {
      /** @description Search results */
      200: {
        content: {
          "application/json": components["schemas"]["ApiSearchResult"];
        };
      };
    };
  };
  sites_export_optic: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["SitesExportOpticParams"];
      };
    };
    responses: {
      /** @description Export site rankings as an optic */
      200: {
        content: {
          "text/plain": string;
        };
      };
    };
  };
  summarize_route: {
    parameters: {
      query: {
        url: string;
        query: string;
      };
    };
    responses: {
      /** @description Summarize a website */
      200: {
        content: {
          "text/plain": string;
        };
      };
    };
  };
  knows_site: {
    parameters: {
      query: {
        site: string;
      };
    };
    responses: {
      /** @description Whether the site is known */
      200: {
        content: {
          "application/json": components["schemas"]["KnowsSite"];
        };
      };
    };
  };
  similar_sites: {
    requestBody: {
      content: {
        "application/json": components["schemas"]["SimilarSitesParams"];
      };
    };
    responses: {
      /** @description List of similar sites */
      200: {
        content: {
          "application/json": components["schemas"]["ScoredSite"][];
        };
      };
    };
  };
}
