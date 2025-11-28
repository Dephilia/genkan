// Doodle theme JavaScript
// Hand-drawn, playful interactions

(function() {
    'use strict';

    // Add loading animation with doodle flair
    document.addEventListener('DOMContentLoaded', function() {
        // Initialize dark mode first (before fade in)
        initializeDarkMode();

        // Fade in effect (no transform on body to preserve fixed positioning)
        document.body.style.opacity = '0';
        setTimeout(function() {
            document.body.style.transition = 'opacity 0.6s ease-in';
            document.body.style.opacity = '1';
        }, 100);

        // Bounce in effect on container
        const container = document.querySelector('.container');
        if (container) {
            container.style.opacity = '0';
            container.style.transform = 'scale(0.95)';
            setTimeout(function() {
                container.style.transition = 'opacity 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55), transform 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55)';
                container.style.opacity = '1';
                container.style.transform = 'scale(1)';
                // Remove transform after animation to ensure no interference with fixed elements
                setTimeout(function() {
                    container.style.transform = '';
                }, 600);
            }, 100);
        }

        // Animate links on load with random bounces
        const links = document.querySelectorAll('.link-button');
        links.forEach(function(link, index) {
            link.style.opacity = '0';
            link.style.transform = 'translateY(30px) rotate(' + (Math.random() * 20 - 10) + 'deg)';
            setTimeout(function() {
                link.style.transition = 'opacity 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55), transform 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55)';
                link.style.opacity = '1';
                link.style.transform = 'translateY(0) rotate(' + (link.style.getPropertyValue('--link-rotation') || '0deg') + ')';
            }, 150 + (index * 80));
        });

        // Animate social links
        const socialLinks = document.querySelectorAll('.social-link, .share-button-inline');
        socialLinks.forEach(function(social, index) {
            social.style.opacity = '0';
            social.style.transform = 'scale(0) rotate(' + (Math.random() * 360) + 'deg)';
            setTimeout(function() {
                social.style.transition = 'opacity 0.4s ease-out, transform 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55)';
                social.style.opacity = '1';
                social.style.transform = 'scale(1) rotate(' + (social.style.getPropertyValue('--random-rotation') || '0deg') + ')';
            }, 300 + (index * 60));
        });

        // Initialize share functionality
        initializeShare();

        // Add doodle cursor trail (optional fun effect)
        addDoodleTrail();

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

            // Add celebration effect
            createCelebration(e.clientX, e.clientY);
        }
    });

    // Add smooth scroll behavior
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

    // Add squiggle effect on button click
    document.querySelectorAll('.link-button').forEach(function(button) {
        button.addEventListener('click', function(e) {
            if (!button.href) return; // Skip if it's a static button

            // Create squiggle ripple
            const squiggle = document.createElement('span');
            const rect = button.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;

            squiggle.style.position = 'absolute';
            squiggle.style.left = x + 'px';
            squiggle.style.top = y + 'px';
            squiggle.style.width = '0';
            squiggle.style.height = '0';
            squiggle.style.borderRadius = '50%';
            squiggle.style.border = '3px dashed var(--primary-color)';
            squiggle.style.opacity = '0.6';
            squiggle.style.pointerEvents = 'none';
            squiggle.style.animation = 'squiggle-expand 0.8s ease-out';

            button.appendChild(squiggle);

            setTimeout(function() {
                squiggle.remove();
            }, 800);
        });
    });

    // Add CSS for squiggle effect
    const style = document.createElement('style');
    style.textContent = `
        @keyframes squiggle-expand {
            0% {
                width: 0;
                height: 0;
                margin-left: 0;
                margin-top: 0;
                opacity: 0.6;
            }
            100% {
                width: 200px;
                height: 200px;
                margin-left: -100px;
                margin-top: -100px;
                opacity: 0;
                transform: rotate(360deg);
            }
        }

        @keyframes celebrate-pop {
            0% {
                transform: translate(-50%, -50%) scale(0) rotate(0deg);
                opacity: 1;
            }
            50% {
                opacity: 1;
            }
            100% {
                transform: translate(-50%, calc(-50% - 100px)) scale(1.5) rotate(720deg);
                opacity: 0;
            }
        }

        .celebrate-star {
            position: fixed;
            pointer-events: none;
            z-index: 9999;
            font-size: 24px;
            animation: celebrate-pop 1s ease-out forwards;
        }
    `;
    document.head.appendChild(style);

    // Create celebration effect
    function createCelebration(x, y) {
        const emojis = ['⭐', '✨', '🌟', '💫', '🎨', '🎪', '🎭'];
        const count = 5;

        for (let i = 0; i < count; i++) {
            setTimeout(function() {
                const star = document.createElement('div');
                star.className = 'celebrate-star';
                star.textContent = emojis[Math.floor(Math.random() * emojis.length)];
                star.style.left = x + (Math.random() * 60 - 30) + 'px';
                star.style.top = y + (Math.random() * 60 - 30) + 'px';
                document.body.appendChild(star);

                setTimeout(function() {
                    star.remove();
                }, 1000);
            }, i * 100);
        }
    }

    // Add doodle cursor trail
    function addDoodleTrail() {
        let lastX = 0;
        let lastY = 0;
        let throttle = false;

        document.addEventListener('mousemove', function(e) {
            if (throttle) return;
            throttle = true;
            setTimeout(function() { throttle = false; }, 100);

            // Only create trail if mouse has moved significantly
            const distance = Math.sqrt(Math.pow(e.clientX - lastX, 2) + Math.pow(e.clientY - lastY, 2));
            if (distance < 50) return;

            lastX = e.clientX;
            lastY = e.clientY;

            // Create a small doodle dot
            const dot = document.createElement('div');
            dot.style.position = 'fixed';
            dot.style.left = e.clientX + 'px';
            dot.style.top = e.clientY + 'px';
            dot.style.width = '4px';
            dot.style.height = '4px';
            dot.style.borderRadius = '50%';
            dot.style.background = 'var(--primary-color)';
            dot.style.opacity = '0.3';
            dot.style.pointerEvents = 'none';
            dot.style.zIndex = '0';
            dot.style.animation = 'fade-out 1s ease-out forwards';
            document.body.appendChild(dot);

            setTimeout(function() {
                dot.remove();
            }, 1000);
        });

        // Add fade-out animation
        const trailStyle = document.createElement('style');
        trailStyle.textContent = `
            @keyframes fade-out {
                to {
                    opacity: 0;
                    transform: scale(2);
                }
            }
        `;
        document.head.appendChild(trailStyle);
    }

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

        // Toggle dark mode on button click with celebration
        toggleButton.addEventListener('click', function(e) {
            const isCurrentlyDark = document.body.classList.contains('dark');

            if (isCurrentlyDark) {
                document.body.classList.remove('dark');
                localStorage.setItem('genkan-theme', 'light');
            } else {
                document.body.classList.add('dark');
                localStorage.setItem('genkan-theme', 'dark');
            }

            // Add mini celebration
            createCelebration(e.clientX, e.clientY);
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
