// SPDX-FileCopyrightText: 2024 Integral <integral@member.fsf.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

async function translate(text, from, to, options) {
    const { config, utils } = options;
    const { tauriFetch: fetch } = utils;
    
    // 獲取配置的 URL 和 API Key
    let { url, apiKey } = config;

    // 如果未設置 URL，使用預設值
    if (!url || url.trim() === '') {
        url = 'https://translate.atomjump.com/';
    }

    // 確保 URL 以 http/https 開頭
    if (!url.startsWith('http')) {
        url = `https://${url}`;
    }

    // 確保 URL 以 / 結尾
    if (!url.endsWith('/')) {
        url += '/';
    }

    const apiUrl = `${url}translate`;

    // 準備請求體
    const body = {
        q: text,
        source: from,
        target: to,
        format: "text",
    };

    // 如果有 API Key，加入請求體
    if (apiKey && apiKey.trim() !== '') {
        body.api_key = apiKey;
    }

    // 發送請求
    const res = await fetch(apiUrl, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: {
            type: 'Json',
            payload: body
        }
    });

    if (res.ok) {
        const result = res.data;
        if (result.translatedText) {
            return result.translatedText;
        } else {
            throw new Error('Translation failed: No translated text returned');
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
} 