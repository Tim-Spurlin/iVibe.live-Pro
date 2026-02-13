const navToggle = document.querySelector('.nav-toggle');
const navLinks = document.querySelector('.nav-links');
const links = Array.from(document.querySelectorAll('.nav-links a'));
const sections = Array.from(document.querySelectorAll('main section[id]'));
const counters = Array.from(document.querySelectorAll('.counter'));
const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

if (navToggle && navLinks) {
  navToggle.addEventListener('click', () => {
    const open = navLinks.classList.toggle('open');
    navToggle.setAttribute('aria-expanded', String(open));
  });
}

const setActive = (id) => {
  links.forEach((link) => {
    const active = link.getAttribute('href') === `#${id}`;
    link.classList.toggle('active', active);
  });
};

const sectionObserver = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.classList.add('in-view');
      setActive(entry.target.id);
    }
  });
}, { threshold: 0.3 });

sections.forEach((section) => sectionObserver.observe(section));

const animateCounter = (el) => {
  const end = Number(el.dataset.target || 0);
  if (reducedMotion || end <= 0) {
    el.textContent = String(end);
    return;
  }
  const duration = 900;
  const start = performance.now();
  const tick = (now) => {
    const progress = Math.min((now - start) / duration, 1);
    el.textContent = String(Math.floor(progress * end));
    if (progress < 1) requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
};

const counterObserver = new IntersectionObserver((entries, observer) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      animateCounter(entry.target);
      observer.unobserve(entry.target);
    }
  });
}, { threshold: 0.6 });

counters.forEach((counter) => counterObserver.observe(counter));

document.getElementById('year').textContent = new Date().getFullYear();
