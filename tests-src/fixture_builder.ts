import {test as base, expect} from '@playwright/test';

export const test = base.extend({
  page: async ({baseURL, page}, use) => {
    await page.goto(baseURL);
    await expect(page).toHaveTitle(/Pendragon Portfolio/);
    await use(page);
  }
})