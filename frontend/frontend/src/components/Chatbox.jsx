import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, Loader2, Sparkles } from 'lucide-react';
import ReactMarkdown from 'react-markdown';

const Chatbot = () => {
  const [input, setInput] = useState('');
  const [messages, setMessages] = useState([
    {
      role: 'ai',
      content: "Hello! I'm your AI assistant. Ask me anything!"
    }
  ]);
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);
  const formatHistory = (msgs) => {
    const history = [];
    for (let i = 1; i < msgs.length - 1; i += 2) {
      if (msgs[i].role === 'user' && msgs[i + 1]?.role === 'ai') {
        history.push({
          user_message: msgs[i].content,
          ai_message: msgs[i + 1].content,
        });
      }
    }
    return history;
  };
  const parseBackendResponse = (rawString) => {
    if (!rawString) return "";

    try {
      return JSON.parse(rawString).response;
    } catch (e) {
      try {
        const firstBraceIndex = rawString.indexOf('{');
        const lastBraceIndex = rawString.lastIndexOf('}');

        if (firstBraceIndex !== -1 && lastBraceIndex !== -1) {
          const jsonSubstring = rawString.substring(firstBraceIndex, lastBraceIndex + 1);
          const parsed = JSON.parse(jsonSubstring);
          return parsed.response;
        }
        console.warn("Could not find JSON braces, returning raw string");
        return rawString;
      } catch (innerError) {
        console.error("Critical parsing error:", innerError);
        return rawString;
      }
    }
  };

  const handleSend = async (e) => {
    e.preventDefault();
    if (!input.trim()) return;

    const currentQuestion = input;
    setInput('');
    
    const newMessages = [...messages, { role: 'user', content: currentQuestion }];
    setMessages(newMessages);
    setIsLoading(true);

    try {
      const payload = {
        current_question: currentQuestion,
        last_messages: formatHistory(newMessages),
      };

      const response = await fetch('http://127.0.0.1:8080/get-answer', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      const result = await response.json();

      if (result.err) {
        throw new Error(result.err);
      }

      const aiContent = parseBackendResponse(result.data);

      setMessages((prev) => [...prev, { role: 'ai', content: aiContent }]);
    } catch (error) {
      console.error(error);
      setMessages((prev) => [
        ...prev,
        { role: 'ai', content: `Sorry, something went wrong. (${error.message})` },
      ]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center w-full h-screen justify-center min-h-screen bg-slate-950 p-4 font-sans text-slate-100">
      <div className="w-full  bg-slate-900 rounded-3xl shadow-2xl overflow-hidden border border-slate-800 flex flex-col h-screen">
        <div className="bg-slate-900 border-b border-slate-800 p-6 flex items-center gap-3 shadow-sm">
          <div className="bg-indigo-600 p-2 rounded-lg shadow-lg shadow-indigo-500/20">
            <Bot className="w-6 h-6 text-white" />
          </div>
          <div>
            <h1 className="text-xl font-bold text-white flex items-center gap-2">
              AI Assistant <Sparkles className="w-4 h-4 text-yellow-400" />
            </h1>
            <p className="text-xs text-slate-400 flex items-center gap-1">
              <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
              Online
            </p>
          </div>
        </div>
        <div className="flex-1 overflow-y-auto p-6 space-y-6 scrollbar-thin scrollbar-thumb-slate-700 scrollbar-track-transparent">
          {messages.map((msg, index) => (
            <div
              key={index}
              className={`flex gap-4 ${msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}`}
            >
              <div className={`flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center ${
                msg.role === 'user' ? 'bg-indigo-600' : 'bg-slate-700'
              }`}>
                {msg.role === 'user' ? <User className="w-6 h-6" /> : <Bot className="w-6 h-6" />}
              </div>

              <div
                className={`max-w-[80%] rounded-2xl px-6 py-4 shadow-md text-sm leading-relaxed ${
                  msg.role === 'user'
                    ? 'bg-indigo-600 text-white rounded-tr-none'
                    : 'bg-slate-800 text-slate-200 rounded-tl-none border border-slate-700'
                }`}
              >
                 {msg.role === 'ai' ? (
                    <div  className="prose prose-invert prose-sm max-w-none prose-p:leading-relaxed prose-pre:bg-slate-900 prose-pre:p-2 prose-pre:rounded-lg">

                    <ReactMarkdown 
                     
                     >
                      {msg.content}
                    </ReactMarkdown>
                        </div>
                 ) : (
                    msg.content
                 )}
              </div>
            </div>
          ))}
          
          {isLoading && (
            <div className="flex gap-4">
              <div className="flex-shrink-0 w-10 h-10 rounded-full bg-slate-700 flex items-center justify-center">
                <Bot className="w-6 h-6" />
              </div>
              <div className="bg-slate-800 rounded-2xl rounded-tl-none px-6 py-4 border border-slate-700 flex items-center gap-2">
                <Loader2 className="w-5 h-5 animate-spin text-indigo-400" />
                <span className="text-slate-400 text-sm">Thinking...</span>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>
        <div className="p-4 bg-slate-900 border-t border-slate-800">
          <form
            onSubmit={handleSend}
            className="flex items-center gap-3 bg-slate-800 p-2 rounded-xl border border-slate-700 focus-within:border-indigo-500 focus-within:ring-1 focus-within:ring-indigo-500 transition-all shadow-inner"
          >
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="Ask a question..."
              className="flex-1 bg-transparent text-white placeholder-slate-400 px-4 py-3 focus:outline-none"
              disabled={isLoading}
            />
            <button
              type="submit"
              disabled={isLoading || !input.trim()}
              className="bg-indigo-600 hover:bg-indigo-500 disabled:bg-slate-700 disabled:cursor-not-allowed text-white p-3 rounded-lg transition-colors duration-200 shadow-lg shadow-indigo-500/30"
            >
              <Send className="w-5 h-5" />
            </button>
          </form>
        </div>
      </div>
    </div>
  );
};

export default Chatbot;