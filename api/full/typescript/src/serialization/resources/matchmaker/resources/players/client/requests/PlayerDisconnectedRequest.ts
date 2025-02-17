/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../../../index";
import * as Tivet from "../../../../../../../api/index";
import * as core from "../../../../../../../core";

export const PlayerDisconnectedRequest: core.serialization.Schema<
    serializers.matchmaker.PlayerDisconnectedRequest.Raw,
    Tivet.matchmaker.PlayerDisconnectedRequest
> = core.serialization.object({
    playerToken: core.serialization.property("player_token", core.serialization.string()),
});

export declare namespace PlayerDisconnectedRequest {
    interface Raw {
        player_token: string;
    }
}
