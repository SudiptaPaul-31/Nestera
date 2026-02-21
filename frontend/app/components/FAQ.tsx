'use client';

import React, { useState, useRef, useEffect } from 'react';
import './FAQ.css';

interface FAQItem {
  question: string;
  answer: string;
}

const faqData: FAQItem[] = [
  {
    question: 'How do I get started with Nestera?',
    answer: 'Getting started with Nestera is simple. Connect your wallet, deposit your preferred stablecoin, and start earning yield immediately. No complex setup required—just a few clicks and you\'re on your way to smarter, on-chain savings.',
  },
  {
    question: 'Can I withdraw my funds at any time?',
    answer: 'Yes, you can withdraw your funds at any time without lock-up periods. Nestera is designed for flexibility, allowing you to access your savings whenever you need them while still earning competitive yields.',
  },
  {
    question: 'Is Nestera audited and safe to use on-chain?',
    answer: 'Absolutely. Nestera\'s smart contracts are thoroughly audited by leading security firms. We prioritize transparency and security, with all code verified on-chain and open for community review.',
  },
  {
    question: 'What stablecoins does Nestera currently support?',
    answer: 'Nestera currently supports major stablecoins including USDC and USDT on the Stellar network. We\'re continuously expanding our supported assets to provide you with more options for your savings strategy.',
  },
];

const FAQ: React.FC = () => {
  const [openIndex, setOpenIndex] = useState<number | null>(null);
  const contentRefs = useRef<(HTMLDivElement | null)[]>([]);
  const [heights, setHeights] = useState<number[]>([]);

  useEffect(() => {
    // Measure the height of each answer content
    const newHeights = contentRefs.current.map((ref) => {
      if (ref) {
        return ref.scrollHeight;
      }
      return 0;
    });
    setHeights(newHeights);
  }, []);

  const toggleItem = (index: number) => {
    setOpenIndex(openIndex === index ? null : index);
  };

  return (
    <section className="faq">
      <div className="faq__container">
        <h2 className="faq__title">Frequently Asked Questions</h2>
        
        <div className="faq__list">
          {faqData.map((item, index) => (
            <div
              key={index}
              className={`faq__item ${openIndex === index ? 'faq__item--open' : ''}`}
            >
              <button
                className="faq__question"
                onClick={() => toggleItem(index)}
                aria-expanded={openIndex === index}
                aria-controls={`faq-answer-${index}`}
              >
                <span className="faq__question-text">{item.question}</span>
                <span className="faq__toggle-icon">
                  {openIndex === index ? '×' : '+'}
                </span>
              </button>
              
              <div
                id={`faq-answer-${index}`}
                className="faq__answer-wrapper"
                style={{
                  height: openIndex === index ? `${heights[index]}px` : '0px',
                }}
              >
                <div
                  className="faq__answer"
                  ref={(el) => {
                    contentRefs.current[index] = el;
                  }}
                >
                  <p className="faq__answer-text">{item.answer}</p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default FAQ;
