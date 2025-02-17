/**
 * This file was auto-generated by Fern from our API Definition.
 */

import * as serializers from "../../../../index";
import * as Tivet from "../../../../../api/index";
import * as core from "../../../../../core";
import { DisplayName } from "../../../common/types/DisplayName";
import { AccountNumber } from "../../../common/types/AccountNumber";
import { Bio } from "../../../common/types/Bio";

export const ValidateProfileRequest: core.serialization.Schema<
    serializers.identity.ValidateProfileRequest.Raw,
    Tivet.identity.ValidateProfileRequest
> = core.serialization.object({
    displayName: core.serialization.property("display_name", DisplayName.optional()),
    accountNumber: core.serialization.property("account_number", AccountNumber.optional()),
    bio: Bio.optional(),
});

export declare namespace ValidateProfileRequest {
    interface Raw {
        display_name?: DisplayName.Raw | null;
        account_number?: AccountNumber.Raw | null;
        bio?: Bio.Raw | null;
    }
}
