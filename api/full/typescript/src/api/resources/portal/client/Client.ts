/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as environments from "../../../../environments";
import * as core from "../../../../core";
import { Games } from "../resources/games/client/Client";

export declare namespace Portal {
    interface Options {
        environment?: core.Supplier<environments.TivetEnvironment | string>;
        token?: core.Supplier<core.BearerToken | undefined>;
        /** Override the X-API-Version header */
        xApiVersion?: "5.1.2";
        fetcher?: core.FetchFunction;
    }

    interface RequestOptions {
        /** The maximum time to wait for a response in seconds. */
        timeoutInSeconds?: number;
        /** The number of times to retry the request. Defaults to 2. */
        maxRetries?: number;
        /** A hook to abort the request. */
        abortSignal?: AbortSignal;
        /** Additional headers to include in the request. */
        headers?: Record<string, string>;
        /** Override the X-API-Version header */
        xApiVersion?: "5.1.2";
    }
}

export class Portal {
    constructor(protected readonly _options: Portal.Options = {}) {}

    protected _games: Games | undefined;

    public get games(): Games {
        return (this._games ??= new Games(this._options));
    }
}
