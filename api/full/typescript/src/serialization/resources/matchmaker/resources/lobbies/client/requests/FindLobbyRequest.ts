/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../index";
import * as Tivet from "../../../../../../../api/index";
import * as core from "../../../../../../../core";
import { Config } from "../../../../../captcha/resources/config/types/Config";

export const FindLobbyRequest: core.serialization.Schema<
    serializers.matchmaker.FindLobbyRequest.Raw,
    Omit<Tivet.matchmaker.FindLobbyRequest, "origin">
> = core.serialization.object({
    gameModes: core.serialization.property("game_modes", core.serialization.list(core.serialization.string())),
    regions: core.serialization.list(core.serialization.string()).optional(),
    preventAutoCreateLobby: core.serialization.property(
        "prevent_auto_create_lobby",
        core.serialization.boolean().optional()
    ),
    tags: core.serialization.record(core.serialization.string(), core.serialization.string()).optional(),
    maxPlayers: core.serialization.property("max_players", core.serialization.number().optional()),
    captcha: Config.optional(),
    verificationData: core.serialization.property("verification_data", core.serialization.unknown().optional()),
});

export declare namespace FindLobbyRequest {
    interface Raw {
        game_modes: string[];
        regions?: string[] | null;
        prevent_auto_create_lobby?: boolean | null;
        tags?: Record<string, string> | null;
        max_players?: number | null;
        captcha?: Config.Raw | null;
        verification_data?: unknown | null;
    }
}
