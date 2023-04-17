// Use Jest to test

import { idlFactory as exampleIDL } from '@/idls/ego_example.idl';
import { _SERVICE as exampleService } from '@/idls/ego_example';
import { getCanisterId, getActor, identity } from '@ego-js/utils';

describe('ego_example', () => {
  test('who am i', async () => {
    const exampleActor =
      // getActor use idl types
      await getActor<exampleService>(
        // use credential identity, owner of canister
        identity(),
        // use idlFactory from generated file
        exampleIDL,
        // get canister ID for 'ego_example', `configs/ego_example.json` is generated
        getCanisterId('ego_example')!,
      );

    const pid = (await exampleActor.whoAmI()).toText();

    expect(pid).toBe(identity().getPrincipal().toText());
  });
});
