import {test as base, expect, mergeTests} from '@playwright/test';
import {basePageFactory, BasePage} from "./POMs/pages";

const overrideTest = base.extend({
  page: async ({baseURL, page}, use) => {
    const headshot = page.getByTestId('Headshot');
    await page.goto(baseURL);
    await expect(page)
      .toHaveTitle(/Pendragon Portfolio/);
    await expect(headshot)
      .toBeVisible();
    await use(page);
  },
})

const fixtureTest = base.extend <
  {
    basePage: BasePage
  }
>({
  basePage: [async ({page}, use) => {
    const bp = basePageFactory(page);
    await use(bp);
  }, {scope: 'test'}]
})

export const test = mergeTests(overrideTest, fixtureTest);