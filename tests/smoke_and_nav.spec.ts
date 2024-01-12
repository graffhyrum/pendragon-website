import {expect} from '@playwright/test';
import {test} from "../tests-src/fixture_builder";
import {goToPage, pages} from "../tests-src/pages";

test('check routes', async ({page}) => {
    for (const route of pages) {
        await goToPage(page, route.name);
        await expect(async () => {
            expect(page.url(), `Page ${route.name} did not load correctly`)
                .toMatch(route.url)
        }).toPass()
        await expect(page.getByRole('heading', {name: route.name}))
            .toBeVisible()
    }
});

test('check links', async ({page}) => {
    for (const route of pages) {
        const navLink = page
            .getByTestId('NavTree')
            .locator('a', {hasText: route.name})
        await expect(navLink, `Link with name ${route.name} should be visible.`).toBeVisible()
        await navLink.click()
        await expect(page.url(),
            `Url for ${route.name} should match ${route.url}.`
        ).toMatch(route.url)
    }
})