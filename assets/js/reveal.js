/*
 * Scroll reveals.
 *
 * The hidden state lives in CSS behind `html.js`, which the inline head script adds before
 * first paint. This file's only jobs are to announce that it loaded, assign stagger
 * delays, and latch elements visible as they enter the viewport. If it never runs, the
 * head script's failsafe strips `.js` and every section is shown.
 */
(function () {
  "use strict";

  window.__revealReady = true;

  var docEl = document.documentElement;

  function showEverything() {
    docEl.className = docEl.className.replace(/(^|\s)js(\s|$)/, "$1$2").trim();
  }

  if (!("IntersectionObserver" in window)) {
    showEverything();
    return;
  }

  // The CSS media query has already neutralised the hidden state; don't build observers.
  if (window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
    return;
  }

  // Stagger children within each group, unless the template set a delay explicitly.
  // Capped so late items in a long list never feel like they're lagging behind the scroll.
  var STEP_MS = 70;
  var MAX_STEPS = 5;
  var groups = document.querySelectorAll("[data-reveal-group]");

  for (var g = 0; g < groups.length; g++) {
    var children = groups[g].querySelectorAll("[data-reveal]");
    for (var c = 0; c < children.length; c++) {
      if (!children[c].style.getPropertyValue("--reveal-delay")) {
        var step = c < MAX_STEPS ? c : MAX_STEPS;
        children[c].style.setProperty("--reveal-delay", step * STEP_MS + "ms");
      }
    }
  }

  var observer = new IntersectionObserver(
    function (entries) {
      for (var i = 0; i < entries.length; i++) {
        if (!entries[i].isIntersecting) continue;
        entries[i].target.classList.add("is-revealed");
        // One-shot: never re-hide on scroll-up.
        observer.unobserve(entries[i].target);
      }
    },
    {
      root: null,
      // threshold 0 + a negative bottom margin, deliberately. A fractional threshold
      // never fires for sections taller than the viewport.
      rootMargin: "0px 0px -10% 0px",
      threshold: 0,
    }
  );

  var targets = document.querySelectorAll("[data-reveal]");
  for (var t = 0; t < targets.length; t++) {
    observer.observe(targets[t]);
  }
})();
