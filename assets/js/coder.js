/*
 * The coder scene.
 *
 * Sections declare `data-scene`; whichever one owns the middle of the viewport picks the
 * pose the figure holds and the code on the screen. The panel is already a complete
 * illustration when this file loads — Askama rendered the first scene into it — so every
 * path below is an enhancement, and failing to run leaves a correct static figure rather
 * than an empty box.
 *
 * Sibling to reveal.js and deliberately in the same dialect: an IIFE, var, no build step.
 */
(function () {
  "use strict";

  var figure = document.querySelector("[data-coder]");
  if (!figure) return;

  var code = figure.querySelector("[data-coder-code]");
  var caption = figure.querySelector("[data-coder-caption]");
  var legend = figure.querySelector("[data-coder-legend]");
  if (!code) return;

  /* Collect the scene library the template rendered alongside the figure. */
  var scenes = {};
  var templates = document.querySelectorAll("template[data-scene]");
  for (var i = 0; i < templates.length; i++) {
    var node = templates[i];
    scenes[node.getAttribute("data-scene")] = {
      pose: node.getAttribute("data-pose") || "typing",
      caption: node.getAttribute("data-caption") || "",
      /* .content is a parsed fragment, so entities are already decoded — the snippets
         contain quotes and angle brackets that innerHTML would hand back escaped. */
      text: node.content.textContent,
    };
  }

  var sections = document.querySelectorAll("[data-scene-section]");
  if (!sections.length) return;

  var reduced =
    window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  var POSES = ["typing", "reviewing", "thinking", "waving"];
  var current = figure.getAttribute("data-current") || "intro";
  var frame = null;
  var settle = null;

  /* ------------------------------------------------------------------ typing ----- */

  /*
   * Retypes the panel. Total duration is capped rather than fixed per character: a long
   * snippet must not still be typing three sections later, and a short one should not
   * finish so fast it reads as a cut.
   */
  var TYPE_MS = 1600;

  function render(scene, animate) {
    if (caption) caption.textContent = scene.caption;
    if (legend) legend.textContent = scene.caption;

    if (frame) {
      cancelAnimationFrame(frame);
      frame = null;
    }

    if (settle) {
      clearTimeout(settle);
      settle = null;
    }

    if (!animate) {
      code.textContent = scene.text;
      return;
    }

    var text = scene.text;
    var started = null;

    function step(now) {
      if (started === null) started = now;
      var progress = Math.min(1, (now - started) / TYPE_MS);
      /* Ease out, so the last characters land rather than stopping dead. */
      var eased = 1 - Math.pow(1 - progress, 2);
      code.textContent = text.slice(0, Math.round(eased * text.length));

      if (progress < 1) {
        frame = requestAnimationFrame(step);
        return;
      }

      frame = null;
      clearTimeout(settle);
      settle = null;
    }

    code.textContent = "";
    frame = requestAnimationFrame(step);

    /*
     * requestAnimationFrame is not a guarantee. An occluded or otherwise unpainted
     * window suspends it while document.visibilityState still reports "visible", which
     * would strand the panel halfway through a word with no way back. Timers keep
     * running there, so one settles the final text regardless of whether a single frame
     * was ever served.
     */
    settle = setTimeout(function () {
      if (frame) cancelAnimationFrame(frame);
      frame = null;
      settle = null;
      code.textContent = text;
    }, TYPE_MS + 250);
  }

  function activate(id) {
    if (id === current) return;
    var scene = scenes[id];
    if (!scene) return;

    current = id;
    figure.setAttribute("data-current", id);
    for (var p = 0; p < POSES.length; p++) {
      figure.classList.toggle("coder--" + POSES[p], POSES[p] === scene.pose);
    }
    render(scene, !reduced && !hidden());
  }

  function hidden() {
    return document.visibilityState === "hidden" || !onScreen;
  }

  /* ------------------------------------------------------------------ tracking ----- */

  /*
   * Which section owns the scene is a question about the whole viewport, not about any
   * one element crossing a line — sections here are frequently taller than the screen, so
   * a plain isIntersecting toggle flickers between neighbours. Instead: on each scroll,
   * pick the section covering the viewport's midpoint.
   */
  var onScreen = true;
  var ticking = false;
  var tick = null;
  var tickTimer = null;

  function pick() {
    if (tick) {
      cancelAnimationFrame(tick);
      tick = null;
    }

    if (tickTimer) {
      clearTimeout(tickTimer);
      tickTimer = null;
    }

    ticking = false;

    /*
     * Coffee drains across the whole read rather than per section — it is the one part
     * of the scene that tracks the journey instead of the current stop. Written as a
     * 0..1 ratio; styles/app.css turns it into a level and fades the steam with it.
     */
    var travel = document.documentElement.scrollHeight - window.innerHeight;
    var drained = travel > 0 ? window.scrollY / travel : 0;
    figure.style.setProperty(
      "--coffee-drain",
      (drained < 0 ? 0 : drained > 1 ? 1 : drained).toFixed(3)
    );

    var mid = window.innerHeight / 2;
    var best = null;
    var bestDistance = Infinity;

    for (var i = 0; i < sections.length; i++) {
      var rect = sections[i].getBoundingClientRect();
      if (rect.top <= mid && rect.bottom >= mid) {
        best = sections[i];
        break;
      }
      /* Above the first section or between two: fall back to the nearest. */
      var distance = rect.top > mid ? rect.top - mid : mid - rect.bottom;
      if (distance < bestDistance) {
        bestDistance = distance;
        best = sections[i];
      }
    }

    /* Above everything, the hero still owns the view. */
    if (best && best.getBoundingClientRect().top > mid) {
      activate("intro");
      return;
    }

    if (best) activate(best.getAttribute("data-scene-section"));
  }

  /*
   * Coalesce to a frame, but never depend on one arriving: where rAF is suspended (see
   * the note on the settle timer above) a rAF-only throttle latches `ticking` on and
   * scene changes stop for good. Whichever of the two fires first cancels the other.
   */
  function onScroll() {
    if (ticking) return;
    ticking = true;
    tick = requestAnimationFrame(pick);
    tickTimer = setTimeout(pick, 250);
  }

  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onScroll, { passive: true });

  /*
   * Idle loops and the typing animation are pure decoration; neither should run against a
   * panel nobody can see. Pausing at the element level rather than the page level also
   * covers the long scroll past the footer.
   */
  if ("IntersectionObserver" in window) {
    new IntersectionObserver(
      function (entries) {
        onScreen = entries[0].isIntersecting;
        figure.classList.toggle("is-idle", !onScreen);
      },
      { threshold: 0 }
    ).observe(figure);
  }

  document.addEventListener("visibilitychange", function () {
    figure.classList.toggle("is-idle", hidden());
  });

  pick();
})();
