
### 1. Structured data (JSON-LD) — the main signal

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "VideoObject",
  "name": "How to Roast Coffee Beans at Home",
  "description": "A 4-minute tutorial that walks you through stovetop roasting.",
  "thumbnailUrl": ["https://example.com/vid/roast-thumb.jpg"],
  "uploadDate": "2025-05-22T08:00:00Z",
  "duration": "PT4M12S",
  "contentUrl": "https://cdn.example.com/vid/roast.mp4",   // file Google can fetch
  "embedUrl":   "https://example.com/vid/roast.html",      // playable page
  "publisher": {
    "@type": "Organization",
    "name": "ExampleCoffee",
    "logo": {
      "@type": "ImageObject",
      "url": "https://example.com/logo.png",
      "width": 600,
      "height": 60
    }
  }
}
</script>
```

*Fields in **bold** in Google docs (`name`, `description`, `thumbnailUrl`, `uploadDate`, `contentUrl`) are required; the rest improve eligibility for richer SERP treatments.* ([developers.google.com][1])

---

### 2. Open Graph / Twitter tags — fall-backs for other platforms

```html
<!-- Open Graph -->
<meta property="og:type"        content="video.other">
<meta property="og:title"       content="How to Roast Coffee Beans">
<meta property="og:description" content="Short tutorial on stovetop roasting.">
<meta property="og:image"       content="https://example.com/vid/roast-thumb.jpg">
<meta property="og:video"       content="https://cdn.example.com/vid/roast.mp4">
<meta property="og:video:type"  content="video/mp4">
<meta property="og:video:width" content="1280">
<meta property="og:video:height"content="720">

<!-- Twitter -->
<meta name="twitter:card"        content="player">
<meta name="twitter:title"       content="How to Roast Coffee Beans">
<meta name="twitter:player"      content="https://example.com/vid/roast.html">
<meta name="twitter:player:width" content="1280">
<meta name="twitter:player:height"content="720">
<meta name="twitter:image"       content="https://example.com/vid/roast-thumb.jpg">
```

These tags don’t influence Google’s moving preview, but they ensure the video unfurls nicely on social platforms.

---

### 3. In-page HTML player (so Google can fetch frames)

```html
<video controls playsinline poster="https://example.com/vid/roast-thumb.jpg">
  <source src="https://cdn.example.com/vid/roast.mp4" type="video/mp4">
</video>
```

Googlebot transcodes the MP4 to create the animated SERP teaser; if the file is blocked, no preview shows. ([developers.google.com][2])

---

### 4. Robots preview controls (optional caps)

```html
<meta name="robots"
      content="max-video-preview:-1, max-image-preview:large, max-snippet:-1">
```

* `max-video-preview:-1` = allow unlimited-length motion preview
* `max-image-preview:large` = let Google choose a big thumbnail
* `max-snippet:-1` = no character cap on the grey text snippet ([blog.google][3])

---

### How it all works together

1. **Crawler fetches** the HTML, the MP4, and the thumbnail.
2. **Structured data** tells Google exactly which file, poster, duration, and landing page belong together; this makes the URL eligible for rich video results. ([schema.org][4])
3. **Robots tag** sets the upper limit on the teaser’s run-time (or lets it run free).
4. At query-time, the **Visual Elements system** assembles the card: blue link, animated clip, thumbnail, and snippet.

Add a high-resolution thumbnail (minimum ▶ ℹ 1280×720 px) and make sure neither the video file nor the image is blocked by `robots.txt`. That’s all you need for Google (and Bing) to show an eye-catching, motion-enabled video preview.

[1]: https://developers.google.com/search/docs/appearance/structured-data/video?utm_source=chatgpt.com "Video (VideoObject, Clip, BroadcastEvent) Schema Markup"
[2]: https://developers.google.com/search/docs/appearance/video?utm_source=chatgpt.com "Video SEO Best Practices | Google Search Central | Documentation"
[3]: https://blog.google/intl/en-in/products/explore-communicate/more-options-to-help-websites-preview/?utm_source=chatgpt.com "More options to help websites preview their content on Google Search"
[4]: https://schema.org/VideoObject?utm_source=chatgpt.com "VideoObject - Schema.org Type"
