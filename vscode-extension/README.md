# إضافة VS Code للغة المرجع
# Al-Marjaa VS Code Extension

<div dir="rtl" align="right">

## نظرة عامة

هذه الإضافة توفر دعماً كاملاً للغة المرجع البرمجية العربية في Visual Studio Code، وتشمل:

- ✨ **تلوين الكود (Syntax Highlighting)**: تلوين كامل للكلمات المفتاحية العربية والإنجليزية
- 🔧 **الإكمال التلقائي (Auto-completion)**: اقتراحات ذكية أثناء الكتابة
- 📖 **التوثيق عند التحويم (Hover Documentation)**: عرض توثيق الكلمات المفتاحية والدوال
- 🐛 **التصحيح (Debugging)**: دعم نقاط التوقف والتنقل خطوة بخطوة
- 📝 **التقطيعات البرمجية (Snippets)**: قوالب جاهزة للدوال والهياكل الشائعة
- 🎯 **خادم اللغة (Language Server)**: تحليل في الوقت الحقيقي للأخطاء

## التثبيت

### من VS Code Marketplace
1. افتح VS Code
2. اضغط `Ctrl+Shift+X` لفتح المتجر
3. ابحث عن "Al-Marjaa"
4. اضغط "تثبيت"

### من المصدر
```bash
cd vscode-extension
npm install
npm run compile
```

## الميزات

### 1. تلوين الكود

يدعم التلوين:
- الكلمات المفتاحية العربية (`دالة`, `متغير`, `إذا`, إلخ)
- الكلمات المفتاحية الإنجليزية (`fn`, `true`, `false`, إلخ)
- الأرقام العربية والإنجليزية (٠١٢٣ و 0123)
- النصوص بين علامات التنصيص
- التعليقات (`#`, `//`, `/* */`)
- الدوال والمتغيرات

### 2. الإكمال التلقائي

اقتراحات ذكية تشمل:
- جميع الكلمات المفتاحية
- الدوال المضمنة
- المتغيرات المعرفة في الملف

### 3. التصحيح

- **نقاط التوقف (Breakpoints)**: إيقاف التنفيذ عند سطر معين
- **التنقل خطوة بخطوة**: `F10` للسطر التالي، `F11` للدخول في الدالة
- **فحص المتغيرات**: عرض قيم المتغيرات الحالية
- **تقييم التعبيرات**: تقييم تعبيرات أثناء التصحيح

### 4. التقطيعات البرمجية

| الاختصار | الوصف |
|---------|-------|
| `دالة` | تعريف دالة جديدة |
| `متغير` | تعريف متغير |
| `ثابت` | تعريف ثابت |
| `إذا` | جملة شرطية |
| `طالما` | حلقة طالما |
| `لكل` | حلقة لكل |
| `حاول` | جملة معالجة الأخطاء |
| `صنف` | تعريف صنف |

## الإعدادات

```json
{
    "marjaa.executablePath": "almarjaa",
    "marjaa.enableLanguageServer": true,
    "marjaa.format.enable": true,
    "marjaa.format.indentSize": 4,
    "marjaa.diagnostics.enable": true,
    "marjaa.completion.enable": true,
    "marjaa.hover.enable": true
}
```

## اختصارات لوحة المفاتيح

| الاختصار | الأمر |
|---------|-------|
| `F5` | تشغيل البرنامج |
| `Ctrl+F5` | تصحيح البرنامج |
| `Shift+Alt+F` | تنسيق الملف |

## أمثلة

### مثال أساسي

```marjaa
# برنامج مرحباً بالعالم
اطبع("مرحباً بالعالم!")؛

# تعريف دالة
دالة ترحيب(الاسم) {
    أرجع "مرحباً " + الاسم + "!"؛
}

# استخدام الدالة
متغير رسالة = ترحيب("أحمد")؛
اطبع(رسالة)؛
```

### مثال مع الفئات

```marjaa
صنف شخص {
    دالة تهيئة(هذا، الاسم، العمر) {
        هذا.الاسم = الاسم؛
        هذا.العمر = العمر؛
    }

    دالة تعريف(هذا) {
        أرجع "أنا " + هذا.الاسم + " وعمري " + هذا.العمر؛
    }
}

متغير أحمد = جديد شخص("أحمد"، 25)؛
اطبع(أحمد.تعريف())؛
```

## المساهمة

نرحب بمساهماتكم! يرجى:
1. عمل Fork للمشروع
2. إنشاء فرع للميزة (`git checkout -b feature/amazing-feature`)
3. عمل Commit للتغييرات (`git commit -m 'إضافة ميزة رائعة'`)
4. رفع الفرع (`git push origin feature/amazing-feature`)
5. فتح Pull Request

## الترخيص

MIT License - انظر ملف [LICENSE](LICENSE) للتفاصيل.

## الروابط

- [المستودع الرئيسي](https://github.com/radhwendalyhamdouni/Al-Marjaa-Core)
- [توثيق اللغة](https://github.com/radhwendalyhamdouni/Al-Marjaa-Core/wiki)
- [الإبلاغ عن مشكلة](https://github.com/radhwendalyhamdouni/Al-Marjaa-Core/issues)

</div>

---

## English Summary

This extension provides full support for the Al-Marjaa Arabic programming language in VS Code, including:

- Complete syntax highlighting for Arabic and English keywords
- Intelligent auto-completion
- Hover documentation
- Debugging support with breakpoints and step-through
- Code snippets for common patterns
- Real-time language server for error detection

</div>
