/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Tivet from "../../../../../../index";

/**
 * @example
 *     {
 *         stream: "std_out",
 *         watchIndex: "string"
 *     }
 */
export interface GetServerLogsRequest {
    stream: Tivet.servers.LogStream;
    /**
     * A query parameter denoting the requests watch index.
     */
    watchIndex?: string;
}
