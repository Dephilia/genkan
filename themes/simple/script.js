// Simple theme JavaScript
// This provides optional enhancements for the link page

(function() {
    'use strict';

    // Add loading animation
    document.addEventListener('DOMContentLoaded', function() {
        // Initialize dark mode first (before fade in)
        initializeDarkMode();

        // Fade in effect
        document.body.style.opacity = '0';
        setTimeout(function() {
            document.body.style.transition = 'opacity 0.5s ease-in';
            document.body.style.opacity = '1';
        }, 100);

        // Animate links on load
        const links = document.querySelectorAll('.link-button');
        links.forEach(function(link, index) {
            link.style.opacity = '0';
            link.style.transform = 'translateY(20px)';
            setTimeout(function() {
                link.style.transition = 'opacity 0.5s ease-out, transform 0.5s ease-out';
                link.style.opacity = '1';
                link.style.transform = 'translateY(0)';
            }, 100 + (index * 50));
        });

        // Initialize share functionality
        initializeShare();

        // Initialize scroll-based dark mode toggle fade
        initializeDarkModeToggleFade();
    });

    // Track link clicks (optional analytics)
    document.addEventListener('click', function(e) {
        const linkButton = e.target.closest('.link-button');
        if (linkButton) {
            const title = linkButton.querySelector('.link-title')?.textContent;
            const url = linkButton.href;

            // Log to console (can be replaced with actual analytics)
            console.log('Link clicked:', {
                title: title,
                url: url,
                timestamp: new Date().toISOString()
            });

            // If you're using analytics, send the event here
            // Example: gtag('event', 'click', { 'link_title': title, 'link_url': url });
        }
    });

    // Add smooth scroll behavior (if needed)
    document.querySelectorAll('a[href^="#"]').forEach(function(anchor) {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });

    // Add ripple effect on button click (optional visual enhancement)
    document.querySelectorAll('.link-button').forEach(function(button) {
        button.addEventListener('click', function(e) {
            const ripple = document.createElement('span');
            const rect = button.getBoundingClientRect();
            const size = Math.max(rect.width, rect.height);
            const x = e.clientX - rect.left - size / 2;
            const y = e.clientY - rect.top - size / 2;

            ripple.style.width = ripple.style.height = size + 'px';
            ripple.style.left = x + 'px';
            ripple.style.top = y + 'px';
            ripple.classList.add('ripple');

            button.appendChild(ripple);

            setTimeout(function() {
                ripple.remove();
            }, 600);
        });
    });

    // Add CSS for ripple effect
    const style = document.createElement('style');
    style.textContent = `
        .link-button {
            position: relative;
            overflow: hidden;
        }
        .ripple {
            position: absolute;
            border-radius: 50%;
            background: rgba(0, 0, 0, 0.1);
            transform: scale(0);
            animation: ripple-animation 0.6s ease-out;
            pointer-events: none;
        }
        @keyframes ripple-animation {
            to {
                transform: scale(4);
                opacity: 0;
            }
        }
    `;
    document.head.appendChild(style);

    // Dark mode functionality
    function initializeDarkMode() {
        const config = window.GENKAN_DARK_MODE || { mode: 'disable' };
        const mode = config.mode.toLowerCase();

        if (mode === 'disable') {
            return;
        }

        const toggleButton = document.getElementById('darkModeToggle');
        if (!toggleButton) {
            return;
        }

        // Check if user has a saved preference
        const savedTheme = localStorage.getItem('genkan-theme');

        // Determine initial dark mode state
        let isDark = false;

        if (savedTheme) {
            // User has a saved preference
            isDark = savedTheme === 'dark';
        } else if (mode === 'dark') {
            // Default to dark
            isDark = true;
        } else if (mode === 'light') {
            // Default to light
            isDark = false;
        } else if (mode === 'auto') {
            // Auto mode: check system preference or time
            if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
                isDark = true;
            } else {
                // Fallback to time-based detection (6 PM to 6 AM is dark)
                const hour = new Date().getHours();
                isDark = hour >= 18 || hour < 6;
            }
        }

        // Apply dark mode
        if (isDark) {
            document.body.classList.add('dark');
        } else {
            document.body.classList.remove('dark');
        }

        // Toggle dark mode on button click
        toggleButton.addEventListener('click', function() {
            const isCurrentlyDark = document.body.classList.contains('dark');

            if (isCurrentlyDark) {
                document.body.classList.remove('dark');
                localStorage.setItem('genkan-theme', 'light');
            } else {
                document.body.classList.add('dark');
                localStorage.setItem('genkan-theme', 'dark');
            }
        });

        // Listen for system theme changes in auto mode
        if (mode === 'auto' && !savedTheme && window.matchMedia) {
            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {
                // Only auto-switch if user hasn't manually set a preference
                if (!localStorage.getItem('genkan-theme')) {
                    if (e.matches) {
                        document.body.classList.add('dark');
                    } else {
                        document.body.classList.remove('dark');
                    }
                }
            });
        }
    }

    // Share functionality
    function initializeShare() {
        const shareButton = document.getElementById('shareButton');
        const shareModal = document.getElementById('shareModal');
        const closeModal = document.getElementById('closeModal');
        const copyButton = document.getElementById('copyButton');
        const shareLink = document.getElementById('shareLink');

        if (!shareButton || !shareModal) return;

        // Open share modal
        shareButton.addEventListener('click', function() {
            shareModal.classList.add('active');
        });

        // Close modal
        if (closeModal) {
            closeModal.addEventListener('click', function() {
                shareModal.classList.remove('active');
            });
        }

        // Close modal when clicking outside
        shareModal.addEventListener('click', function(e) {
            if (e.target === shareModal) {
                shareModal.classList.remove('active');
            }
        });

        // Close modal with Escape key
        document.addEventListener('keydown', function(e) {
            if (e.key === 'Escape' && shareModal.classList.contains('active')) {
                shareModal.classList.remove('active');
            }
        });

        // Copy link to clipboard
        if (copyButton && shareLink) {
            copyButton.addEventListener('click', function() {
                // Try modern clipboard API
                if (navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard.writeText(shareLink.value).then(function() {
                        copyButton.textContent = 'Copied!';
                        copyButton.classList.add('copied');
                        setTimeout(function() {
                            copyButton.textContent = 'Copy';
                            copyButton.classList.remove('copied');
                        }, 2000);
                    }).catch(function(err) {
                        console.error('Clipboard API failed:', err);
                        // Fallback to execCommand
                        fallbackCopy();
                    });
                } else {
                    // Fallback for older browsers
                    fallbackCopy();
                }

                function fallbackCopy() {
                    try {
                        shareLink.select();
                        shareLink.setSelectionRange(0, 99999); // For mobile devices
                        var copied = document.execCommand('copy');
                        if (copied) {
                            copyButton.textContent = 'Copied!';
                            copyButton.classList.add('copied');
                            setTimeout(function() {
                                copyButton.textContent = 'Copy';
                                copyButton.classList.remove('copied');
                            }, 2000);
                        }
                    } catch (e) {
                        console.error('Copy failed:', e);
                    }
                }
            });
        }
    }

    // Dark mode toggle fade on scroll
    function initializeDarkModeToggleFade() {
        const toggleButton = document.getElementById('darkModeToggle');
        if (!toggleButton) return;

        let lastScrollY = window.scrollY;
        let ticking = false;

        // Handle scroll with RAF for better performance
        function handleScroll() {
            lastScrollY = window.scrollY;

            if (!ticking) {
                window.requestAnimationFrame(function() {
                    updateToggleOpacity(lastScrollY);
                    ticking = false;
                });
                ticking = true;
            }
        }

        // Update opacity based on scroll position
        function updateToggleOpacity(scrollY) {
            // Fade out starts at 100px, fully transparent by 300px
            const fadeStart = 100;
            const fadeEnd = 300;

            if (scrollY <= fadeStart) {
                // Fully visible at top
                toggleButton.style.opacity = '1';
                toggleButton.style.pointerEvents = 'auto';
            } else if (scrollY >= fadeEnd) {
                // Fully hidden after fadeEnd
                toggleButton.style.opacity = '0';
                toggleButton.style.pointerEvents = 'none';
            } else {
                // Gradual fade between fadeStart and fadeEnd
                const opacity = 1 - ((scrollY - fadeStart) / (fadeEnd - fadeStart));
                toggleButton.style.opacity = opacity.toString();
                toggleButton.style.pointerEvents = opacity > 0.1 ? 'auto' : 'none';
            }
        }

        // Set initial opacity
        updateToggleOpacity(window.scrollY);

        // Add scroll listener
        window.addEventListener('scroll', handleScroll, { passive: true });
    }

})();
