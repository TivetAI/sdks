/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as Tivet from "../../../../../../index";

/**
 * @example
 *     {
 *         gameMode: "string",
 *         region: "string",
 *         publicity: "public",
 *         tags: {
 *             "string": "string"
 *         },
 *         maxPlayers: 1,
 *         lobbyConfig: {
 *             "key": "value"
 *         },
 *         captcha: {
 *             hcaptcha: {
 *                 clientResponse: "string"
 *             },
 *             turnstile: {
 *                 clientResponse: "string"
 *             }
 *         },
 *         verificationData: {
 *             "key": "value"
 *         }
 *     }
 */
export interface CreateLobbyRequest {
    gameMode: string;
    region?: string;
    publicity?: Tivet.matchmaker.CustomLobbyPublicity;
    tags?: Record<string, string>;
    maxPlayers?: number;
    lobbyConfig?: unknown;
    captcha?: Tivet.captcha.Config;
    verificationData?: unknown;
}
