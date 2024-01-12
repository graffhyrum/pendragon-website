import {expect} from '@playwright/test';
import {test} from "../tests-src/fixture_builder";

test('check routes', async ({page, basePage}) => {
    for (const route of basePage.pages) {
        await basePage.goTo(route.name)
        await expect(async () => {
            expect(page.url(), `Page ${route.name} did not load correctly`)
                .toMatch(route.url)
        }).toPass()
        await expect(page.getByRole('heading', {name: route.name}))
            .toBeVisible()
    }
});

test('check links', async ({page, basePage}) => {
    for (const route of basePage.pages) {
        const navLink = basePage.getNavLink(route.name)
        await expect(navLink, `Link with name ${route.name} should be visible.`)
            .toBeVisible()
        await navLink.click()
        await expect(page.url(),
            `Url for ${route.name} should match ${route.url}.`
        ).toMatch(route.url)
    }
})